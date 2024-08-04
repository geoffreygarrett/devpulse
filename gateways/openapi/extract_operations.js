const fs = require('fs');
const path = require('path');
const semver = require('semver');
const openapiExtract = require('openapi-extract');

const RESERVED_NAMES = ['model', 'Model'];
const CUSTOM_RENAME_MAPPING = {
    'Model': 'ApiModel',
    'model': 'api_model',
};

function renameIfReserved(name) {
    return RESERVED_NAMES.includes(name) ? CUSTOM_RENAME_MAPPING[name] || `Renamed${name}` : name;
}

function renameSchemaProperties(schema) {
    if (schema.properties) {
        Object.entries(schema.properties).forEach(([key, value]) => {
            const newKey = renameIfReserved(key);
            if (newKey !== key) {
                schema.properties[newKey] = value;
                delete schema.properties[key];
            }
            renameSchemaProperties(value);
        });
    }

    if (schema.items) {
        renameSchemaProperties(schema.items);
    }
}

function updateReferences(schema, originalName, newName) {
    if (schema.$ref && schema.$ref.endsWith(`/schemas/${originalName}`)) {
        schema.$ref = schema.$ref.replace(`/schemas/${originalName}`, `/schemas/${newName}`);
    }
    if (schema.properties) {
        Object.values(schema.properties).forEach(property => updateReferences(property, originalName, newName));
    }
    if (schema.items) {
        updateReferences(schema.items, originalName, newName);
    }
}

function renameAndFixComponents(components) {
    const renamedComponents = {};
    if (components && components.schemas) {
        Object.entries(components.schemas).forEach(([key, value]) => {
            const renamedKey = renameIfReserved(key);
            renamedComponents[renamedKey] = value;
            renameSchemaProperties(value);
            updateReferences(value, key, renamedKey);
            // Ensure type is set correctly for schemas
            if (!value.type) {
                value.type = 'object';
            }
            // Handle schema defaults correctly
            if (value.default !== undefined && typeof value.default !== 'object') {
                value.default = {};
            }
        });
    }
    return renamedComponents;
}

function detectCollisionsInOneOf(schema) {
    if (schema.oneOf) {
        const typeCount = schema.oneOf.reduce((acc, item) => {
            const type = item.type;
            if (type) {
                if (!acc[type]) {
                    acc[type] = [];
                }
                acc[type].push(item);
            }
            return acc;
        }, {});

        Object.entries(typeCount).forEach(([type, items]) => {
            if (items.length > 1) {
                items.forEach(item => {
                    if (type === 'array') {
                        if (item.items.type === 'string') {
                            item['x-variant-name'] = 'StringVector';
                        } else if (item.items.type === 'integer') {
                            if (item.items.items && item.items.items.type === 'integer') {
                                item['x-variant-name'] = 'NestedIntegerVector';
                            } else {
                                item['x-variant-name'] = 'IntegerVector';
                            }
                        }
                    } else if (type === 'string') {
                        item['x-variant-name'] = 'String';
                    }
                });
            }
        });
    }

    if (schema.properties) {
        Object.values(schema.properties).forEach(property => detectCollisionsInOneOf(property));
    }

    if (schema.items) {
        detectCollisionsInOneOf(schema.items);
    }
}

function detectCollisionsInPaths(paths) {
    Object.values(paths).forEach(methods => {
        Object.values(methods).forEach(operation => {
            if (operation.requestBody && operation.requestBody.content) {
                Object.values(operation.requestBody.content).forEach(content => {
                    if (content.schema) {
                        detectCollisionsInOneOf(content.schema);
                    }
                });
            }
            if (operation.responses) {
                Object.values(operation.responses).forEach(response => {
                    if (response.content) {
                        Object.values(response.content).forEach(content => {
                            if (content.schema) {
                                detectCollisionsInOneOf(content.schema);
                            }
                        });
                    }
                });
            }
            if (operation.parameters) {
                operation.parameters.forEach(parameter => {
                    if (parameter.schema) {
                        detectCollisionsInOneOf(parameter.schema);
                    }
                });
            }
        });
    });
}

async function readAndParseSpec(inputFile) {
    let data = fs.readFileSync(inputFile, 'utf8');
    if (data.charCodeAt(0) === 0xFEFF) {
        data = data.slice(1);
    }
    const spec = JSON.parse(data);

    if (spec.info && spec.info.version) {
        const parsedVersion = semver.valid(semver.coerce(spec.info.version));
        if (!parsedVersion) {
            throw new Error(`Invalid version format: ${spec.info.version}`);
        }
        spec.info.version = parsedVersion;
    }
    return spec;
}

function addVendorOperationIds(spec, separator) {
    Object.entries(spec.paths).forEach(([path, methods]) => {
        Object.entries(methods).forEach(([method, operation]) => {
            if (!operation.operationId) {
                return;
            }
            const operationId = operation.operationId;
            const parts = separator ? operationId.split(separator) : [operationId];
            if (parts.length > 1) {
                operation["x-sub-operation-id"] = parts.slice(1).join(separator).replace(/\s|-/g, '_').toLowerCase();
                operation["x-group"] = parts[0].replace(/\s|-/g, '_').toLowerCase();
            } else {
                operation["x-sub-operation-id"] = operationId.replace(/\s|-/g, '_').toLowerCase();
                operation["x-group"] = operation.tag ? operation.tag.replace(/\s|-/g, '_').toLowerCase() : operation.tags[0].replace(/\s|-/g, '_').toLowerCase();
            }
        });
    });
}

function collectOperationIds(spec, config) {
    let operationIds = [];
    const {includeAllOperationIds, extractByTags, targetTag} = config;
    Object.entries(spec.paths).forEach(([path, methods]) => {
        Object.entries(methods).forEach(([method, operation]) => {
            const operationId = operation.operationId;
            if (operationId) {
                const shouldInclude = includeAllOperationIds ||
                    (extractByTags && operation.tags && operation.tags.includes(targetTag));

                if (shouldInclude) {
                    operationIds.push(operationId);
                }
            }
        });
    });
    return operationIds;
}

async function extractOperations(inputFile, outputFile, options, cacheableOperations, separator, config) {
    try {
        const spec = await readAndParseSpec(inputFile);
        addVendorOperationIds(spec, separator);
        const operationIds = collectOperationIds(spec, config);
        options.operationid = [...operationIds, ...options.operationid];


        if (config.excludeOperationIds) {
            options.operationid = options.operationid.filter(id => !config.excludeOperationIds.includes(id));
        }

        // Detect collisions in oneOf in paths
        detectCollisionsInPaths(spec.paths);

        if (spec.components && spec.components.schemas) {
            // spec.components.schemas = renameAndFixComponents(spec.components);
            // Detect collisions in oneOf in components
            Object.values(spec.components.schemas).forEach(schema => detectCollisionsInOneOf(schema));
        } else {
            console.warn('No schemas found in the specification.');
        }

        const extractedData = openapiExtract.extract(spec, options);
        fs.writeFileSync(outputFile, JSON.stringify(extractedData, null, 2), 'utf8');
        console.log('Extraction complete. Data saved to:', outputFile);
    } catch (error) {
        console.error('Failed to extract operations:', error);
        throw error;
    }
}


module.exports = {extractOperations};
