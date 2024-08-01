const {extractOperations} = require('../extract_operations');
const path = require('path');

const inputFile = path.join(__dirname, 'azure_spec.json'); // Path to the input OpenAPI spec file for Azure
const outputFile = path.join(__dirname, 'azure_spec.extracted.json'); // Path where the extracted spec will be saved

const options = {
    openai: true, // Assuming this refers to a specific configuration in your extraction tool
    server: true, // Assuming this enables server-related extraction details
    security: true, // Assuming this enables security configurations in the extraction
    operationid: [
        // Commit related endpoints
        "Commits_Get",
        "Commits_Get Changes",
        "Commits_List",
        "Commits_Get Stats",
        "Commits_Get Batch",
        "Commits_Get Unchanged"
    ]
};

const cacheableOperations = {
    "Commits_Get": 3600, // Cache for 1 hour
    "Commits_Get Changes": 600, // Cache for 10 minutes
    "Commits_List": 600, // Cache for 10 minutes
    // Add other operations with respective caching times
};

const config = {
    includeAllOperationIds: false,
    extractByTags: false,
    targetTag: '',
    transformOperation: null,
    transformSchema: null,
    logErrors: true
};

extractOperations(inputFile, outputFile, options, cacheableOperations, '_', config)
    .then(() => console.log('Extraction complete! Output saved to:', outputFile))
    .catch(err => console.error('Failed to extract operations:', err));
