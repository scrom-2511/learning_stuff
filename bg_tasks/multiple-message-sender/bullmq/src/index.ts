import express from "express";
import { enqueueEmailNoti } from "./services/email/producer.js";
import { enqueueDiscordNoti } from "./services/discord/producer.js";
import { enqueueTelegramNoti } from "./services/telegram/producer.js";

const app = express();

for (let i = 0; i < 20; i++) {
    await Promise.all([enqueueEmailNoti(i), enqueueDiscordNoti(i), enqueueTelegramNoti(i)]);
}

app.listen(3000, () => {
    console.log("listeninig at 3000")
})