let openapiFilter = require('openapi-filter');
let fs = require('fs');
//options.inverse = false;
//options.valid = false;
//options.flags = ['x-internal'];


obj = JSON.parse(fs.readFileSync('openai_spec.json', 'utf8'));



let options = {
    inverse: true,
    valid: false,
    info: true,
    flags: ['x-internal']
};


let res = openapiFilter.filter(obj, options);
fs.writeFileSync('openai_spec_filtered.json', JSON.stringify(res, null, 2));