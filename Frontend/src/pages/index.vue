<script setup lang="ts">
import { ref } from 'vue'

interface PaymentFormData {
  order_number: string
  amount: number
  success_url: string
  failure_url: string
  cust_name: string
  email_id: string
  mobile_no: string
}

interface PaymentResponse {
  post_url: string
  me_id: string
  merchant_request: string
  hash: string
}

const isCheckoutLoading = ref(false)

// Test data matching the YagoutPay documentation example
const paymentFormData: PaymentFormData = {
  order_number: '49340',  // Using the exact example from docs
  amount: 1,  // Using the exact example from docs
  success_url: 'http://127.0.0.1:5173/success',
  failure_url: 'http://127.0.0.1:5173/failure',
  cust_name: 'John Doe',
  email_id: 'john@example.com',
  mobile_no: '0912345678'
}

const handleCheckout = async () => {
  try {
    isCheckoutLoading.value = true
    console.log('Payment form data being sent:', paymentFormData)

    const response = await fetch('http://127.0.0.1:8000/create_payment', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(paymentFormData),
    })

    if (!response.ok) throw new Error(await response.text())

    const data: PaymentResponse = await response.json()
    console.log('Payment response received:', data)
    console.log('Hash value:', data.hash)
    console.log('Hash length:', data.hash.length)

    // Create and submit the payment form to YagoutPay
    const form = document.createElement('form')
    form.method = 'POST'
    form.action = data.post_url
    form.style.display = 'none'

    // Add the required fields exactly as per YagoutPay documentation
    ;[
      { name: 'me_id', value: data.me_id },
      { name: 'merchant_request', value: data.merchant_request },
      { name: 'hash', value: data.hash },
    ].forEach(f => {
      const input = document.createElement('input')
      input.type = 'hidden'
      input.name = f.name
      input.value = f.value
      form.appendChild(input)
    })

    document.body.appendChild(form)

    console.log('Form ready to submit:', form)
    console.log('Form inputs:', Array.from(form.elements).map(e => ({ name: e.name, value: e.value })))

    form.submit() // leave commented until ready
  } catch (err: any) {
    console.error('Checkout failed:', err)
    alert(`Checkout failed: ${err.message}`)
  } finally {
    isCheckoutLoading.value = false
  }
}

</script>




<template
    >
<div id="webcrumbs">
        <div class="bg-neutral-50 min-h-screen">
            <div class="container mx-auto px-4 py-8">
                <div class="flex flex-col lg:flex-row gap-8">
                    <div class="w-full lg:w-3/5">
                        <div class="bg-white p-4 rounded-lg">
                            <div class="relative h-[400px] mb-4">
                                <img
                                    src="https://images.unsplash.com/photo-1603891128711-11b4b03bb138"
                                    alt="iPhone 15 Pro"
                                    class="w-full h-full object-cover rounded-lg"
                                    keywords="iphone, smartphone, mobile phone, apple"
                                />
                                <span class="absolute top-4 left-4 bg-primary-500 text-white px-4 py-2 font-bold"
                                    >NEW</span
                                >
                            </div>
                            <div class="grid grid-cols-4 gap-2">
                                <div
                                    class="h-24 border-2 border-primary-500 rounded-lg overflow-hidden transition-transform hover:scale-105"
                                >
                                    <img
                                        src="https://imgs.search.brave.com/J2_GewSYaLe5b0ifUOV7kVC4mgmcpntzwraZEvlP9_c/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly9zczcu/dnp3LmNvbS9pcy9p/bWFnZS9WZXJpem9u/V2lyZWxlc3MvYXBw/bGUtaXBob25lLTE1/LXByby1tYXgtMjU2/Z2ItbmF0dXJhbC10/aXRhbml1bS1tdTY4/M2xsLWEtYT9oZWk9/MjUwJmJnYz1mNmY2/ZjY"
                                        class="w-full h-full object-cover"
                                    />
                                </div>
                                <div
                                    class="h-24 border-2 border-transparent rounded-lg overflow-hidden hover:border-primary-500 transition-all hover:scale-105"
                                >
                                    <img
                                        src="https://images.unsplash.com/photo-1592750475338-74b7b21085ab"
                                        alt="iPhone in hand"
                                        class="w-full h-full object-cover"
                                        keywords="iphone, hand, using"
                                    />
                                </div>
                                <div
                                    class="h-24 border-2 border-transparent rounded-lg overflow-hidden hover:border-primary-500 transition-all hover:scale-105"
                                >
                                    <img
                                        src="https://images.unsplash.com/photo-1605236453806-6ff36851218e"
                                        alt="iPhone side view"
                                        class="w-full h-full object-cover"
                                        keywords="iphone, side view"
                                    />
                                </div>
                                <div
                                    class="h-24 border-2 border-transparent rounded-lg overflow-hidden hover:border-primary-500 transition-all hover:scale-105"
                                >
                                    <img
                                        src="https://imgs.search.brave.com/SstrsEOCtUBCnx4N6qVpLWr9FvDm-z6rq8s-XhBvEGc/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly9jZG4u/bW9zLmNtcy5mdXR1/cmVjZG4ubmV0L2V2/OFRmZnN6RzdMenRy/Rm5CNVRxc2ouanBn"
                                        class="w-full h-full object-cover"
                                    />
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="w-full lg:w-2/5">
                        <div class="bg-white p-8 rounded-lg h-full flex flex-col">
                            <div class="mb-6">
                                <h1 class="text-3xl font-bold mb-2">iPhone 15 Pro Max</h1>
                                <div class="flex items-center mb-4">
                                    <div class="flex">
                                        <span class="text-yellow-400">★</span><span class="text-yellow-400">★</span
                                        ><span class="text-yellow-400">★</span><span class="text-yellow-400">★</span
                                        ><span class="text-yellow-400">★</span>
                                    </div>
                                    <span class="ml-2 text-gray-600">(126 reviews)</span>
                                </div>
                                <div class="flex items-center mb-6">
                                    <span class="text-4xl font-bold">$1,099.99</span>
                                    <span class="ml-2 text-gray-500 line-through">$249.99</span>
                                    <span
                                        class="ml-2 bg-primary-100 text-primary-800 px-2 py-1 text-sm font-semibold rounded"
                                        >SAVE 20%</span
                                    >
                                </div>
                            </div>
                            <div class="mb-6">
                                <h2 class="text-xl font-bold mb-4">Select Color</h2>
                                <div class="flex space-x-4">
                                    <div
                                        class="w-12 h-12 bg-neutral-900 rounded-lg border-2 border-primary-500 hover:scale-110 transition-transform cursor-pointer"
                                    ></div>
                                    <div
                                        class="w-12 h-12 bg-blue-700 rounded-lg border-2 border-transparent hover:border-primary-500 hover:scale-110 transition-transform cursor-pointer"
                                    ></div>
                                    <div
                                        class="w-12 h-12 bg-rose-500 rounded-lg border-2 border-transparent hover:border-primary-500 hover:scale-110 transition-transform cursor-pointer"
                                    ></div>
                                    <div
                                        class="w-12 h-12 bg-neutral-200 rounded-lg border-2 border-transparent hover:border-primary-500 hover:scale-110 transition-transform cursor-pointer"
                                    ></div>
                                </div>
                            </div>
                            <div class="mb-8">
                                <h2 class="text-xl font-bold mb-2">Key Features</h2>
                                <ul class="space-y-2">
                                    <li class="flex items-center">
                                        <span class="material-symbols-outlined text-primary-500 mr-2"
                                            >check_circle</span
                                        >
                                        A17 Pro chip with 6-core CPU
                                    </li>
                                    <li class="flex items-center">
                                        <span class="material-symbols-outlined text-primary-500 mr-2"
                                            >check_circle</span
                                        >
                                        48MP main camera with 5x optical zoom
                                    </li>
                                    <li class="flex items-center">
                                        <span class="material-symbols-outlined text-primary-500 mr-2"
                                            >check_circle</span
                                        >
                                        Water resistant (IP68 rated)
                                    </li>
                                    <li class="flex items-center">
                                        <span class="material-symbols-outlined text-primary-500 mr-2"
                                            >check_circle</span
                                        >
                                        GPS &amp; 20+ sport modes
                                    </li>
                                </ul>
                            </div>
                            <div class="mt-auto">
                                <div class="flex items-center mb-6">
                                    <div class="border border-gray-300 rounded-lg flex items-center mr-4">
                                        <button
                                            class="px-4 py-2 text-lg hover:bg-gray-100 rounded-l-lg transition-colors"
                                        >
                                            -
                                        </button>
                                        <span class="px-4 py-2 text-lg font-semibold">1</span>
                                        <button
                                            class="px-4 py-2 text-lg hover:bg-gray-100 rounded-r-lg transition-colors"
                                        >
                                            +
                                        </button>
                                    </div>
                                    <div class="text-green-600 flex items-center">
                                        <span class="material-symbols-outlined mr-1">inventory</span> In Stock
                                    </div>
                                </div>
                                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                                    <button
                                        class="bg-white border-2 border-primary-500 text-primary-500 font-bold py-4 px-6 rounded-lg hover:bg-primary-50 transition-colors flex items-center justify-center"
                                    >
                                        <span class="material-symbols-outlined mr-2">favorite</span> Add to Wishlist
                                    </button>
                                    <button
                                        class="bg-primary-500 text-white font-bold py-4 px-6 rounded-lg hover:bg-primary-600 transition-colors flex items-center justify-center"
                                    >
                                        <span class="material-symbols-outlined mr-2">shopping_cart</span> Add to Cart
                                    </button>
                                </div>
                                <button
                                @click="handleCheckout"
                                :disabled="isCheckoutLoading"
                                class="w-full mt-4 bg-primary-800 text-white font-bold py-5 px-6 rounded-lg hover:bg-primary-900 transition-transform hover:scale-[1.02] flex items-center justify-center disabled:opacity-50 disabled:cursor-not-allowed"
                                >
                                <span v-if="isCheckoutLoading" class="inline-flex items-center">
                                    <svg class="animate-spin -ml-1 mr-2 h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                    Processing...
                                </span>
                                <span v-else>
                                    <span class="material-symbols-outlined mr-2">payments</span> Checkout Now
                                </span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="mt-12">
                    <div class="bg-white rounded-lg overflow-hidden">
                        <div class="border-b border-gray-200">
                            <div class="flex overflow-x-auto">
                                <button
                                    class="px-6 py-4 font-bold text-primary-500 border-b-2 border-primary-500 hover:bg-gray-50 transition-colors"
                                >
                                    Description
                                </button>
                                <button
                                    class="px-6 py-4 font-bold text-gray-500 hover:text-primary-500 border-b-2 border-transparent hover:border-primary-500 hover:bg-gray-50 transition-colors"
                                >
                                    Specifications
                                </button>
                                <button
                                    class="px-6 py-4 font-bold text-gray-500 hover:text-primary-500 border-b-2 border-transparent hover:border-primary-500 hover:bg-gray-50 transition-colors"
                                >
                                    Reviews
                                </button>
                                <button
                                    class="px-6 py-4 font-bold text-gray-500 hover:text-primary-500 border-b-2 border-transparent hover:border-primary-500 hover:bg-gray-50 transition-colors"
                                >
                                    Shipping
                                </button>
                            </div>
                        </div>
                        <div class="p-6">
                            <p class="text-gray-700 mb-4">
                                The iPhone 15 Pro Max is Apple’s flagship smartphone, combining cutting-edge performance with
                                a sleek, durable design. It’s perfect for tech enthusiasts who demand speed, style, and innovation.
                            </p>
                            <p class="text-neutral-700 mb-4">
                                The stunning 6.7-inch Super Retina XDR display with ProMotion technology delivers incredibly
                                fluid scrolling, vivid video playback, and responsive gaming. The always-on display keeps your
                                essential information visible at a glance.
                            </p>
                            <p class="text-gray-700">
                                Built for an active lifestyle, the iPhone 15 Pro Max features advanced durability with Ceramic
                                Shield, water and dust resistance, and a powerful A17 Pro chip for unmatched performance.
                                Enjoy long battery life and next-level photography with the pro camera system.
                            </p>
                        </div>
                    </div>
                </div>
                <div class="mt-12">
                    <h2 class="text-2xl font-bold mb-6">You May Also Like</h2>
                    <div class="grid grid-cols-1 md:grid-cols-3 lg:grid-cols-4 gap-6">
                        <div class="bg-white rounded-lg overflow-hidden hover:shadow-lg transition-shadow">
                            <div class="h-48 overflow-hidden">
                                <img
                                    src="https://imgs.search.brave.com/31XkM9bKmOCZp56J1PA08q2lRliQX3tfw68wADBbC6E/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly9pLmlu/c2lkZXIuY29tLzY2/YmEyOTJjNWRhNDA2/Mzk3YmY0ZmY1Zj93/aWR0aD03MDA"
                                    alt="Related product 1"
                                    class="w-full h-full object-cover hover:scale-105 transition-transform"
                                    keywords="tech products, gadgets, electronics"
                                />
                            </div>
                            <div class="p-4">
                                <h3 class="font-bold mb-2">AirPods Pro 2</h3>
                                <div class="flex justify-between items-center">
                                    <span class="font-bold">$249.99</span>
                                    <button
                                        class="w-10 h-10 bg-primary-100 text-primary-500 rounded-full flex items-center justify-center hover:bg-primary-500 hover:text-white transition-colors"
                                    >
                                        <span class="material-symbols-outlined">add_shopping_cart</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="bg-white rounded-lg overflow-hidden hover:shadow-lg transition-shadow">
                            <div class="h-48 overflow-hidden">
                                <img
                                    src="https://images.unsplash.com/photo-1537589376225-5405c60a5bd8"
                                    alt="Related product 2"
                                    class="w-full h-full object-cover hover:scale-105 transition-transform"
                                    keywords="tech products, gadgets, electronics"
                                />
                            </div>
                            <div class="p-4">
                                <h3 class="font-bold mb-2">Iphone Xs</h3>
                                <div class="flex justify-between items-center">
                                    <span class="font-bold">$399.99</span>
                                    <button
                                        class="w-10 h-10 bg-primary-100 text-primary-500 rounded-full flex items-center justify-center hover:bg-primary-500 hover:text-white transition-colors"
                                    >
                                        <span class="material-symbols-outlined">add_shopping_cart</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="bg-white rounded-lg overflow-hidden hover:shadow-lg transition-shadow">
                            <div class="h-48 overflow-hidden">
                                <img
                                    src="https://imgs.search.brave.com/bs4TvKd2aAnY0eQgmvONkodPsUu_Dm2gLNRX0EoCYJA/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly93d3cu/ZXNydGVjaC5jb20v/Y2RuL3Nob3AvZmls/ZXMvMTVXLUNhci1D/aGFyZ2VyLXdpdGgt/TWFnU2FmZS1Dcnlv/Qm9vc3QtQzAxLVpU/MDEuanBnP3Y9MTc0/MjIwNTQ3NSZ3aWR0/aD0xMTAw"
                                    alt="Related product 3"
                                    class="w-full h-full object-cover hover:scale-105 transition-transform"
                                    keywords="tech products, gadgets, electronics"
                                />
                            </div>
                            <div class="p-4">
                                <h3 class="font-bold mb-2">MagSafe Charger</h3>
                                <div class="flex justify-between items-center">
                                    <span class="font-bold">$39.99</span>
                                    <button
                                        class="w-10 h-10 bg-primary-100 text-primary-500 rounded-full flex items-center justify-center hover:bg-primary-500 hover:text-white transition-colors"
                                    >
                                        <span class="material-symbols-outlined">add_shopping_cart</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                        <div class="bg-white rounded-lg overflow-hidden hover:shadow-lg transition-shadow">
                            <div class="h-48 overflow-hidden">
                                <img
                                    src="https://imgs.search.brave.com/MaLsOfwnTHkzEAIpBp2_VLoCQaXwO5gAxz074srq7jw/rs:fit:500:0:1:0/g:ce/aHR0cHM6Ly9pNS53/YWxtYXJ0aW1hZ2Vz/LmNvbS9zZW8vMjAy/NC1BcHBsZS0xMy1p/bmNoLWlQYWQtQWly/LU0yLUJ1aWx0LWZv/ci1BcHBsZS1JbnRl/bGxpZ2VuY2UtV2kt/RmktMTI4R0ItQmx1/ZV8xNTEyNjEyYi0z/ZjgxLTQ2NjEtOWI1/YS1iNmUzMDUwMWZm/OTkuMmNhZmYzNzBi/MzRiMmE5NWNkZTUy/NTA5ZTAyZjY4Yjku/anBlZz9vZG5IZWln/aHQ9NTczJm9kbldp/ZHRoPTU3MyZvZG5C/Zz1GRkZGRkY"
                                    alt="Related product 4"
                                    class="w-full h-full object-cover hover:scale-105 transition-transform"
                                    keywords="tech products, gadgets, electronics"
                                />
                            </div>
                            <div class="p-4">
                                <h3 class="font-bold mb-2">iPad Air</h3>
                                <div class="flex justify-between items-center">
                                    <span class="font-bold">$599.99</span>
                                    <button
                                        class="w-10 h-10 bg-primary-100 text-primary-500 rounded-full flex items-center justify-center hover:bg-primary-500 hover:text-white transition-colors"
                                    >
                                        <span class="material-symbols-outlined">add_shopping_cart</span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div></template
>



<style scoped>
/* Add any custom styles here */
</style>
