const path = require('path');
const {extractOperations} = require('../extract_operations');
const fs = require('fs');
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

function renameComponents(components) {
    return Object.entries(components).reduce((acc, [key, value]) => {
        const renamedKey = renameIfReserved(key);
        acc[renamedKey] = value;
        renameSchemaProperties(value);
        return acc;
    }, {});
}

const inputFile = path.join(__dirname, 'openai_spec.json');
const outputFile = path.join(__dirname, 'openai_spec.extracted.json');

const options = {
    openai: true,
    server: true,
    security: true,
    operationid: []
};

const cacheableOperations = {};

// const config = {
//     includeAllOperationIds: true,
//     extractByTags: false,
//     transformSchema: renameSchemaProperties,
// };
//
// extractOperations(inputFile, outputFile, options, cacheableOperations, null, config)
//     .then(() => console.log('Extraction complete!'))
//     .catch(err => console.error('Error during extraction:', err));


const config = {
    includeAllOperationIds: true,
    extractByTags: false,
    targetTag: '',
    logErrors: true,
    excludeOperationIds: [
        "createImage",
        "createAssistant",
        "createThreadAndRun",
        "createFineTuningJob",
        "createModeration",
        "createTranscription",
        "createTranslation",
        "createImageEdit",
        "createCompletion",
        "createSpeech",
        "createEmbedding",
        "createImageVariation"
    ]
};


extractOperations(inputFile, outputFile, options, cacheableOperations, null, config)
    .then(() => console.log('Extraction complete! Output saved to:', outputFile))
    .catch(err => console.error('Failed to extract operations:', err));
