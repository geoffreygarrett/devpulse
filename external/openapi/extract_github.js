const { extractOperations } = require('./extract_operations');
const path = require('path');

const inputFile = path.join(__dirname, 'api.github.com.json'); // Adjust the path to your input file
const outputFile = path.join(__dirname, 'api.github.com.extracted.json'); // Adjust the path to your output file

const options = {
    openai: true,
    server: true,
    security: true,
    operationid: [
        // Commit related endpoints
        "repos/list-commits",
        "repos/list-comments-for-commit",
        "repos/create-commit-comment",
        "repos/list-pull-requests-associated-with-commit",
        "repos/get-commit",
        "checks/list-for-ref",
        "repos/list-commit-statuses-for-ref",
        "repos/list-commit-comments-for-repo",

        // Pull request related endpoints
        "pulls/list",
        "pulls/create",
        "pulls/get",
        "pulls/update",
        "pulls/list-review-comments",
        "pulls/create-review-comment",
        "pulls/update-review-comment",
        "pulls/delete-review-comment",
        "pulls/list-reviews",
        "pulls/create-review",
        "pulls/dismiss-review",
        "pulls/submit-review",
        "pulls/merge",

        // Issue related endpoints
        "issues/list-comments",
        "issues/create-comment",
        "issues/update-comment",
        "issues/delete-comment",

        // Content related endpoints
        "repos/get-content",

        // User related endpoints
        "users/get-authenticated",
        "users/get-by-username",
        "users/list-followers-for-authenticated-user",
        "users/list-followers-for-user",
        "users/list-following-for-authenticated-user",
        "users/list-following-for-user",
        "users/check-person-is-followed-by-authenticated",
        "users/list-public-keys-for-authenticated",
        "users/list-public-keys-for-user",
        "users/list-gpg-keys-for-authenticated",
        "users/list-gpg-keys-for-user",

        // Repository related endpoints
        "repos/list-for-authenticated-user",
        "repos/list-for-user",
        "repos/get",
        "repos/list-branches",
        "repos/get-branch",
        "repos/list-tags",
        "repos/get-content",
        "repos/create-file",
        "repos/update-file",
        "repos/delete-file",

        // Organization related endpoints
        "orgs/get",
        "orgs/list-for-authenticated-user",
        "orgs/list-members",
        "orgs/check-public-member"
    ]
};

const cacheableOperations = {
    "repos/list-commits": 600, // 10 minutes
    "repos/list-comments-for-commit": 10, // 10 seconds
    "repos/create-commit-comment": 0, // No caching
    "repos/list-pull-requests-associated-with-commit": 600, // 10 minutes
    "repos/get-commit": 3600, // 1 hour
    "checks/list-for-ref": 600, // 10 minutes
    "repos/list-commit-statuses-for-ref": 600, // 10 minutes
    "repos/list-commit-comments-for-repo": 600, // 10 minutes
    // Add other operations with their respective cache durations as needed
};

extractOperations(inputFile, outputFile, options, cacheableOperations, '/').then(r => console.log('Done!'));
