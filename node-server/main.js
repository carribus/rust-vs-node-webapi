const express = require('express');
const app = express();
const port = 3000;

const index = (req, res) => {
    res.send("Rust-vs-node benchmark<br/>Node-server benchmark server");
}

const get_fib = (req, res) => {
    var a = 1, b = 0, temp;
    let num = req.params.length || 0;

    while (num >= 0){
      temp = a;
      a = a + b;
      b = temp;
      num--;
    }
  
    res.send(`${b}`);
}

app.get('/', index);
app.get('/fib/:length', get_fib)

app.listen(port, () => console.log(`Rust-vs-node: NODE SERVER listening on port ${port}`));

