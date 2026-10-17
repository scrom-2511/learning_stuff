import { Queue, Worker } from "bullmq"

const queue = new Queue("email");

export const enqueueEmailNoti = async (userId: number) => {
    await queue.add("sendEmail", { userId })
}