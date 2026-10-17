import express from "express";
import { enqueueNotifications } from "./producer";
import "./setup"

const app = express();

await enqueueNotifications(`${1} message produced`);

app.listen(3000)
