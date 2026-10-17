import amqplib from "amqplib"

export const connection = await amqplib.connect('amqp://localhost:5672')