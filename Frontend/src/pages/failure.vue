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
        <div class="min-h-screen bg-red-50 flex items-center justify-center p-4">
            <div class="bg-white p-8 max-w-md w-full text-center">
                <div class="mx-auto w-16 h-16 bg-red-100 flex items-center justify-center mb-6">
                    <svg class="w-8 h-8 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path
                            stroke-linecap="round"
                            stroke-linejoin="round"
                            stroke-width="2"
                            d="M6 18L18 6M6 6l12 12"
                        ></path>
                    </svg>
                </div>
                <h1 class="text-2xl font-bold mb-4">Payment Failed</h1>
                <p class="text-gray-600 mb-6">
                    Unfortunately, your payment could not be processed. Please try again or contact support if the issue
                    persists.
                </p>
                <div class="bg-red-50 border-2 border-red-200 p-4 mb-6 text-left">
                    <h3 class="font-bold mb-2">Error Details</h3>
                    <div class="space-y-1 text-sm">
                        <div class="flex justify-between">
                            <span>Order Number:</span> <span class="font-bold">ORD-49340</span>
                        </div>
                        <div class="flex justify-between">
                            <span>Amount:</span> <span class="font-bold">ETB 1.00</span>
                        </div>
                        <div class="flex justify-between">
                            <span>Status:</span> <span class="text-red-600 font-bold">Failed</span>
                        </div>
                        <div class="mt-2 p-2 bg-red-100 text-xs">Payment processing failed - Insufficient funds</div>
                    </div>
                </div>
                <div class="space-y-3">
                    <button
                        @click="handler_0_handler"
                        class="w-full bg-red-600 text-white font-bold py-3 px-6 hover:bg-red-700 transition-colors"
                    >
                        Try Again
                    </button>
                    <button
                        @click="handler_1_handler"
                        class="w-full bg-white border-2 border-red-600 text-red-600 font-bold py-3 px-6 hover:bg-red-50 transition-colors"
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


