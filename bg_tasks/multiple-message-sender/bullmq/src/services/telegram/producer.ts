import { Queue } from "bullmq";

const queue = new Queue("telegram");

export const enqueueTelegramNoti = async (userId: number) => {
    await queue.add("sendTelegram", { userId });
};
