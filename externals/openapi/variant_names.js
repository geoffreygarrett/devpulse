const path = require('path');

function generateVariantName(item) {
    if (item.$ref) {
        return item.$ref.split('/').pop();
    } else if (item.type === 'string') {
        return 'StringVariant';
    } else if (item.type === 'array') {
        if (item.items.type === 'string') {
            return 'StringArrayVariant';
        } else if (item.items.type === 'integer') {
            return 'IntegerArrayVariant';
        } else if (item.items.type === 'array') {
            return 'NestedArrayVariant';
        }
    }
    return 'UnknownVariant';
}

function processSchema(schema, variantCounts) {
    for (const key of ['oneOf', 'anyOf', 'allOf']) {
        if (schema[key]) {
            schema[key].forEach(item => {
                const variantName = generateVariantName(item);
                const suffix = variantCounts[variantName] > 1 ? variantCounts[variantName] : '';
                item['x-variant-name'] = variantName + suffix;
                variantCounts[variantName]++;
                processSchema(item, variantCounts);
            });
        }
    }

    if (schema.properties) {
        Object.values(schema.properties).forEach(subSchema => processSchema(subSchema, variantCounts));
    }

    if (schema.items) {
        processSchema(schema.items, variantCounts);
    }
}

function addVariantNames(schema) {
    const variantCounts = {};
    processSchema(schema, variantCounts);
    return schema;
}

module.exports = addVariantNames;
