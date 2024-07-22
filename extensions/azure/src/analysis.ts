// src/analysis.ts
import * as https from 'https';

function analyzeRepository(startCommit: string, endCommit: string, repositoryUrl: string) {
    const postData = JSON.stringify({
        start_commit: startCommit,
        end_commit: endCommit,
        repository_url: repositoryUrl
    });

    const options = {
        hostname: 'devpulse.shuttleapp.rs',
        port: 443,
        path: '/repository/commit-range',
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        }
    };

    const req = https.request(options, (res) => {
        console.log(`STATUS: ${res.statusCode}`);
        res.setEncoding('utf8');
        res.on('data', (chunk) => {
            console.log(`BODY: ${chunk}`);
        });
    });

    req.on('error', (e) => {
        console.error(`problem with request: ${e.message}`);
    });

    // write data to request body
    req.write(postData);
    req.end();
}
