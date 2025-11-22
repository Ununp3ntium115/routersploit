// PyRouterSploit Scanner Node for Node-RED

module.exports = function(RED) {
    function ScannerNode(config) {
        RED.nodes.createNode(this, config);
        const node = this;

        node.on('input', async function(msg) {
            const target = msg.payload.target || config.target;
            const scanType = msg.payload.scanType || config.scanType || 'autopwn';
            const threads = msg.payload.threads || config.threads || 10;

            node.status({fill: "blue", shape: "dot", text: `scanning ${target}`});

            try {
                // Call PyRouterSploit REST API
                const axios = require('axios');
                const response = await axios.post('http://localhost:8080/api/scan', {
                    target: target,
                    scan_type: scanType,
                    threads: threads
                });

                msg.payload = {
                    target: target,
                    scan_id: response.data.scan_id,
                    status: 'initiated',
                    timestamp: new Date().toISOString()
                };

                node.status({fill: "green", shape: "dot", text: "scan initiated"});
                node.send(msg);
            } catch (error) {
                node.status({fill: "red", shape: "ring", text: "error"});
                node.error(`Scanner error: ${error.message}`, msg);
            }
        });
    }

    RED.nodes.registerType("pyroutersploit-scanner", ScannerNode);
};
