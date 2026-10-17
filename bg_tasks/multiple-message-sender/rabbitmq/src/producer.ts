import channel, { exchange } from "./exchange";

export async function enqueueNotifications(msg: string) {
    await channel.publish(exchange, "", Buffer.from(msg)); // 2nd param is routing key, empty key means fanout exchange.
}