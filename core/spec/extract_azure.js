const { extractOperations } = require('./extract_operations');
const path = require('path');

const inputFile = path.join(__dirname, 'dev.azure.com.json'); // Adjust the path to your input file
const outputFile = path.join(__dirname, 'dev.azure.com.extracted.json'); // Adjust the path to your output file

const options = {
    openai: true,
    server: true,
    security: true,
    operationid: [
        // Commit related endpoints
        "Commits_Get",
        "Commits_Get Changes",
        "Commits_List",
        "Commits_Get Stats",
        "Commits_Get Batch",
        "Commits_Get Unchanged"
        // Add more endpoints as needed
    ]
};

const cacheableOperations = {
    "Commits_Get": 3600, // 1 hour
    "Commits_Get Changes": 600, // 10 minutes
    "Commits_List": 600, // 10 minutes
    // Add other operations with their respective cache durations as needed
};

extractOperations(inputFile, outputFile, options, cacheableOperations, '_').then(r => console.log('Done!'));
