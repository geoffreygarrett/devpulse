const openApiDocument = require('./azure/git.json');

Object.keys(openApiDocument.paths).forEach(path => {
    const methods = Object.keys(openApiDocument.paths[path]);
    console.log(`Endpoint: ${path}`);
    methods.forEach(method => {
        console.log(`  Method: ${method}`);
    });
});