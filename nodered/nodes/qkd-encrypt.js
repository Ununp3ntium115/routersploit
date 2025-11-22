// PyRouterSploit QKD Encryption Node for Node-RED

module.exports = function(RED) {
    function QKDEncryptNode(config) {
        RED.nodes.createNode(this, config);
        const node = this;

        node.on('input', async function(msg) {
            const data = msg.payload.data || msg.payload;
            const keySize = config.keySize || 32;

            node.status({fill: "blue", shape: "dot", text: "encrypting"});

            try {
                const axios = require('axios');
                const response = await axios.post('http://localhost:8080/api/qkd/encrypt', {
                    data: String(data),
                    key_size: keySize
                });

                msg.payload = {
                    original: data,
                    ciphertext: response.data.ciphertext,
                    session_id: response.data.session_id,
                    algorithm: 'QKD-BB84-Hybrid',
                    timestamp: new Date().toISOString()
                };

                node.status({fill: "green", shape: "dot", text: "encrypted"});
                node.send(msg);
            } catch (error) {
                node.status({fill: "red", shape: "ring", text: "error"});
                node.error(`QKD encryption error: ${error.message}`, msg);
            }
        });
    }

    RED.nodes.registerType("pyroutersploit-qkd-encrypt", QKDEncryptNode);
};
