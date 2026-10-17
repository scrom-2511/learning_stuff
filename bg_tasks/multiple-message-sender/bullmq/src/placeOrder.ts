import { inventoryUpdateEnqueue } from "./services/inventory/inventory.js";
import { sendNotificationEnqueue } from "./services/notification/notification.js";
import { processPayment } from "./services/payment/payment.js";

export async function placeOrder(productId: string) {
    let paymentSuccessfull = await processPayment();

    let userId = 1;

    if (paymentSuccessfull) {
        await Promise.all([
            inventoryUpdateEnqueue(productId),
            sendNotificationEnqueue(userId),
        ]);
    }
}