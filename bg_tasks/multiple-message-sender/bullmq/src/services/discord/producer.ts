import { Queue } from "bullmq";

const queue = new Queue("discord");

export const enqueueDiscordNoti = async (userId: number) => {
    await queue.add("sendDiscord", { userId });
};
