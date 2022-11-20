import express from "express";

const app = express();
const PING_LISTEN_PORT = 3000;

app.get("/ping", (req, res) => {
  res.json(req.headers);
});

app.use(function (req, res, next) {
  var err = new Error("Not Found");
  res.status(404).end(); // using response here
  next(err);
});

app.listen(PING_LISTEN_PORT, () => {
  return console.log(
    `Express is listening at http://localhost:${PING_LISTEN_PORT}`
  );
});