const express = require('express');
const path = require('path');
const app = express();

app.use(express.static(path.join(__dirname, 'html')));

app.get('/', async(req, res) => {
    res.sendFile(path.join(__dirname, 'html', 'index.html'));
});

app.listen(3000, () => {
    console.log("Server successfully running on port 3000");
});