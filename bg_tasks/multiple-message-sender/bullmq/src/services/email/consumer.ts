import { Worker } from "bullmq";
import { Redis } from "ioredis";

const connection = new Redis({ maxRetriesPerRequest: null });

const worker = new Worker(
    "email",
    async (job) => {
        const random = Math.floor(Math.random() * (3000 - 1000 + 1)) + 1000;
        await new Promise((resolve) => setTimeout(resolve, random));

        console.log("email sent for user: ", job.data)
    },
    { connection }
);