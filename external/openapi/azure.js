const openapiExtract = require('openapi-extract');
const fs = require('fs');
const path = require('path');

async function extractAzureDevOpsOperations() {
    const inputFile = path.join(__dirname, './azure/git.json'); // Path to the Azure DevOps OpenAPI spec file
    const outputFile = path.join(__dirname, 'extracted-azure-operations.json'); // Where to save the extracted operations

    try {
        // Read the OpenAPI spec file for Azure DevOps
        const openAPISpec = fs.readFileSync(inputFile, 'utf8');

        // Define the options for extraction, focusing on DevOps related operations
        const options = {
            openai: true,
            operationid: [
                // Repository management endpoints
                "git/repositories/getRepository",
                "git/repositories/createRepository",
                "git/repositories/deleteRepository",
                "git/repositories/updateRepository",

                // Commit management endpoints
                "git/commits/getCommits",
                "git/commits/getCommit",
                "git/commits/createCommit",

                // Pull request management endpoints
                "git/pullRequests/getPullRequests",
                "git/pullRequests/getPullRequest",
                "git/pullRequests/createPullRequest",
                "git/pullRequests/updatePullRequest",
                "git/pullRequests/mergePullRequest",

                // Build and release endpoints
                "build/builds/getBuilds",
                "build/builds/queueBuild",
                "release/releases/createRelease",
                "release/releases/updateRelease",

                // Work item tracking
                "work/workItems/getWorkItems",
                "work/workItems/updateWorkItem",

                // Additional useful endpoints for DevOps analysis
                "git/policyConfigurations/getPolicyConfigurations",
                "git/refs/getRefs",
                "git/statuses/getStatuses"
            ]
        };

        // Extract the operations based on specified operation IDs
        const extractedData = openapiExtract.extract(JSON.parse(openAPISpec), options);

        // Write the extracted data to a file
        fs.writeFileSync(outputFile, JSON.stringify(extractedData, null, 2), 'utf8');
        console.log('Extraction complete. Data saved to ' + outputFile);
    } catch (error) {
        console.error('Failed to extract operations:', error);
    }
}

extractAzureDevOpsOperations();
