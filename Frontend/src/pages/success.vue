<script>
    export default {
        methods: {
            handler_0_handler() {
                return (window.location.href = "/")
            },
            handler_1_handler() {
                return (window.location.href = "/")
            }
        }
    }
</script>

<template
    ><div id="webcrumbs">
        <div class="min-h-screen bg-emerald-50 flex items-center justify-center p-4">
            <div class="bg-white p-8 max-w-md w-full text-center rounded-lg shadow-lg">
                <div class="mx-auto w-16 h-16 bg-emerald-100 rounded-full flex items-center justify-center mb-6">
                    <svg class="w-8 h-8 text-emerald-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                    </svg>
                </div>
                <h1 class="text-2xl font-bold mb-4">Payment Successful</h1>
                <p class="text-gray-600 mb-6">
                    Your payment has been successfully processed. Thank you for your purchase!
                </p>
                <div class="bg-emerald-50 border-2 border-emerald-200 p-4 mb-6 text-left rounded">
                    <h3 class="font-bold mb-2">Transaction Details</h3>
                    <div class="space-y-1 text-sm">
                        <div class="flex justify-between">
                            <span>Order Number:</span> <span class="font-bold">ORD-49340</span>
                        </div>
                        <div class="flex justify-between">
                            <span>Amount:</span> <span class="font-bold">ETB 1.00</span>
                        </div>
                        <div class="flex justify-between">
                            <span>Status:</span> <span class="text-emerald-600 font-bold">Completed</span>
                        </div>
                        <div class="mt-2 p-2 bg-emerald-100 text-xs rounded">
                            Payment successfully processed - Thank you
                        </div>
                    </div>
                </div>
                <div class="space-y-3">
                    <button
                        @click="handler_0_handler"
                        class="w-full bg-emerald-600 text-white font-bold py-3 px-6 hover:bg-emerald-700 transition-colors rounded"
                    >
                        View Receipt
                    </button>
                    <button
                        @click="handler_1_handler"
                        class="w-full bg-white border-2 border-emerald-600 text-emerald-600 font-bold py-3 px-6 hover:bg-emerald-50 transition-colors rounded"
                    >
                        Back to Home
                    </button>
                </div>
                <div class="mt-6 text-xs text-gray-500">
                    <p class="font-bold">Common reasons for payment failure:</p>
                    <ul class="mt-2 text-left space-y-1">
                        <li>• Insufficient funds in your account</li>
                        <li>• Incorrect payment details</li>
                        <li>• Network connectivity issues</li>
                        <li>• Bank declined the transaction</li>
                    </ul>
                    <p class="mt-3 font-bold">Need help? Contact our support team at support@example.com</p>
                </div>
            </div>
        </div>
    </div></template
>


