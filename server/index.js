const express = require("express")

for (let i = 0; i < 5; i++) {
    const app = express()
    app.get("/", (req, res) => {
        res.set("Connection", "close")
        res.send(`Hello from server ${i + 1}!`)
    })
    app.listen(3000 + i + 1, () => {
        console.log(`Base Server ${i + 1} is running on port ${3000 + i + 1}`)
    })
}