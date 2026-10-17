import express from "express";
import { enqueueNotifications } from "./producer";
import "./setup"

const app = express();

for (let i = 0; i < 20; i++) {
    await enqueueNotifications(`${i} message produced`);
}

app.listen(3000)
