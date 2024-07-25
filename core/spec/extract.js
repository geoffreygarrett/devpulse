const openapiExtract = require('openapi-extract');
const fs = require('fs');
const path = require('path');

async function extractOperations() {
    const inputFile = path.join(__dirname, 'github.yaml'); // Adjust the path to your input file
    const outputFile = path.join(__dirname, 'extracted-operations.json'); // Adjust the path to your output file

    try {
        // Read the OpenAPI spec file
        const openAPISpec = fs.readFileSync(inputFile, 'utf8');

        // Define the options for extraction
        const options = {
            openai: true,
            operationid: [
                "repos/list-commits",
                "repos/list-comments-for-commit",
                "repos/create-commit-comment",
                "repos/list-pull-requests-associated-with-commit",
                "repos/get-commit",
                "checks/list-for-ref",
                "repos/list-commit-statuses-for-ref",
                "repos/list-commit-comments-for-repo",
                "pull-request/assigned",
                "pull-request/auto-merge-disabled",
                "pull-request/auto-merge-enabled",
                "pull-request/closed",
                "pull-request/converted-to-draft",
                "pull-request/demilestoned",
                "pull-request/dequeued",
                "pull-request/edited",
                "pull-request/enqueued",
                "pull-request/labeled",
                "pull-request/locked",
                "pull-request/milestoned",
                "pull-request/opened",
                "pull-request/ready-for-review",
                "pull-request/reopened",
                "pull-request-review-comment/created"
            ]
        };

        // Extract the operations
        const extractedData = openapiExtract.extract(JSON.parse(openAPISpec), options);

        // Write the extracted data to a file
        fs.writeFileSync(outputFile, JSON.stringify(extractedData, null, 2), 'utf8');
        console.log('Extraction complete. Data saved to ' + outputFile);
    } catch (error) {
        console.error('Failed to extract operations:', error);
    }
}

extractOperations();
