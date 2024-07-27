const openapiExtract = require('openapi-extract');
const fs = require('fs');
const semver = require('semver');

async function extractOperations(inputFile, outputFile, options, cacheableOperations, separator, includeAllOperationIds = false) {
    try {
        // Read the OpenAPI spec file and remove BOM if present
        let openAPISpec = fs.readFileSync(inputFile, 'utf8');
        if (openAPISpec.charCodeAt(0) === 0xFEFF) {
            openAPISpec = openAPISpec.slice(1);
        }

        // Parse the OpenAPI spec
        let rawSpec = JSON.parse(openAPISpec);

        // Process version to make it compatible with Cargo and SemVer
        if (rawSpec.info && rawSpec.info.version) {
            // Attempt to parse version using semver
            let parsedVersion = semver.valid(semver.coerce(rawSpec.info.version));
            if (!parsedVersion) {
                throw new Error(`Invalid version format: ${rawSpec.info.version}`);
            }
            rawSpec.info.version = parsedVersion;
        }

        // Collect all operation IDs if includeAllOperationIds is true
        let operationIds = [];
        if (includeAllOperationIds) {
            for (const path in rawSpec.paths) {
                for (const method in rawSpec.paths[path]) {
                    const operation = rawSpec.paths[path][method];
                    if (operation.operationId) {
                        operationIds.push(operation.operationId);

                        const operationParts = operation.operationId.split(separator);
                        if (operationParts.length > 1) {
                            const subOperationId = operationParts.slice(1).join(separator)
                                .replace(/\s/g, '_')
                                .replace(/-/g, '_')
                                .toLowerCase();
                            const group = operationParts[0]
                                .replace(/\s/g, '_')
                                .replace(/-/g, '_')
                                .toLowerCase();
                            operation["x-sub-operation-id"] = subOperationId;
                            operation["x-group"] = group;
                        }
                    }
                }
            }
        }

        // Iterate over extracted data and add caching information
        for (const path in rawSpec.paths) {
            for (const method in rawSpec.paths[path]) {
                const operation = rawSpec.paths[path][method];
                const cacheTime = cacheableOperations[operation.operationId];
                if (cacheTime !== undefined) {
                    operation["x-cache"] = {
                        enabled: true,
                        params: {
                            ty: "SizedCache<String, String>",
                            create: "{ SizedCache::with_size(100) }",
                            convert: "{ format!(\"{}\", key) }",
                            time: cacheTime
                        }
                    };
                }
            }
        }

        // Set options to include all operation IDs if flag is true
        if (includeAllOperationIds) {
            options.operationid = operationIds;
        }

        const extractedData = openapiExtract.extract(rawSpec, options);
        fs.writeFileSync(outputFile, JSON.stringify(extractedData, null, 2), 'utf8');
        console.log('Extraction complete. Data saved to ' + outputFile);
    } catch (error) {
        console.error('Failed to extract operations:', error);
    }
}

module.exports = { extractOperations };
