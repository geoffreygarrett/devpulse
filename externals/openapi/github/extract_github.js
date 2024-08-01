const {extractOperations} = require('../extract_operations');
const path = require('path');

const inputFile = path.join(__dirname, 'github_spec.json'); // Path to the input OpenAPI spec file for GitHub
const outputFile = path.join(__dirname, 'github_spec.extracted.json'); // Path where the extracted spec will be saved

const options = {
    openai: true, // Assuming this refers to a specific configuration in your extraction tool
    server: true, // Assuming this enables server-related extraction details
    security: true, // Assuming this enables security configurations in the extraction
    operationid: [
        "repos/list-commits",
        "repos/list-comments-for-commit",
        "repos/create-commit-comment",
        "repos/list-pull-requests-associated-with-commit",
        "repos/get-commit",
        "checks/list-for-ref",
        "repos/list-commit-statuses-for-ref",
        "repos/list-commit-comments-for-repo",
        "pulls/list",
        "pulls/create",
        "pulls/get",
        // Add more as listed above
        "orgs/get",
        "orgs/list-for-authenticated-user",
        "orgs/list-members",
        "orgs/check-public-member"
    ]
};

const cacheableOperations = {
    "repos/list-commits": 600, // Cache for 10 minutes
    "repos/list-comments-for-commit": 10, // Cache for 10 seconds
    "repos/create-commit-comment": 0, // No caching
    "repos/list-pull-requests-associated-with-commit": 600, // Cache for 10 minutes
    "repos/get-commit": 3600, // Cache for 1 hour
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


extractOperations(inputFile, outputFile, options, cacheableOperations, '/', config)
    .then(() => console.log('Extraction complete! Output saved to:', outputFile))
    .catch(err => console.error('Failed to extract operations:', err));
