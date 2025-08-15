<template>
  <div class="min-h-screen bg-gray-100 flex items-center justify-center p-4">
    <!-- Payment Modal -->
    <div class="bg-white rounded-lg shadow-2xl w-full max-w-md mx-auto">
      <!-- Header Section -->
      <div class="border-b border-gray-200 p-6">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center space-x-3">
            <!-- YagoutPay Logo -->
            <div class="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center">
              <span class="text-white font-bold text-xl">Y</span>
            </div>
            <span class="text-xl font-semibold text-gray-900">YagoutPay SDK</span>
          </div>
          
          <!-- Security Badge -->
          <div class="flex items-center space-x-2">
            <div class="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center">
              <svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
              </svg>
            </div>
            <span class="text-sm font-medium text-green-600">Yagout Secure Payments</span>
          </div>
        </div>
        
        <!-- Close Button and Time -->
        <div class="flex items-center justify-between">
          <button class="text-gray-400 hover:text-gray-600">
            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
          <span class="text-sm text-gray-500">{{ currentTime }}</span>
        </div>
      </div>

      <!-- Payment Form -->
      <div class="p-6">
        <form @submit.prevent="handleSubmit" class="space-y-6">
          <!-- Wallets Section -->
          <div>
            <h2 class="text-lg font-semibold text-gray-900 mb-4">Wallets</h2>
            
            <!-- Phone Number Input -->
            <div class="mb-6">
              <label class="block text-sm font-medium text-gray-700 mb-2">Phone Number</label>
              <div class="relative">
                <div class="absolute left-3 top-1/2 transform -translate-y-1/2">
                  <!-- Ethiopia Flag -->
                  <div class="w-6 h-4 bg-gradient-to-r from-green-500 via-yellow-500 to-red-500 rounded-sm relative">
                    <div class="absolute inset-0 bg-blue-600 rounded-full w-2 h-2 top-1 left-1"></div>
                    <div class="absolute inset-0 text-white text-xs font-bold top-0 left-2">★</div>
                  </div>
                </div>
                <input
                  v-model="formData.mobile_no"
                  type="tel"
                  required
                  class="pl-12 pr-12 w-full border-2 border-gray-300 rounded-lg py-3 focus:border-blue-500 focus:ring-2 focus:ring-blue-200 transition-colors"
                  placeholder="0911223344"
                />
                <div class="absolute right-3 top-1/2 transform -translate-y-1/2">
                  <div class="w-6 h-6 bg-green-500 rounded-full flex items-center justify-center">
                    <svg class="w-4 h-4 text-white" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                    </svg>
                  </div>
                </div>
              </div>
            </div>

            <!-- Payment Method Selection -->
            <div class="space-y-3">
              <div class="grid grid-cols-3 gap-3">
                <!-- Telebirr -->
                <div 
                  @click="selectedPaymentMethod = 'telebirr'"
                  :class="[
                    'relative border-2 rounded-lg p-3 cursor-pointer transition-all hover:scale-105',
                    selectedPaymentMethod === 'telebirr' 
                      ? 'border-green-500 bg-green-50' 
                      : 'border-gray-200 hover:border-gray-300'
                  ]"
                >
                  <div class="text-center">
                    <div class="w-12 h-12 bg-green-500 rounded-lg mx-auto mb-2 flex items-center justify-center">
                      <span class="text-white font-bold text-xl">t</span>
                    </div>
                    <div class="text-xs font-medium text-gray-700">thNC telebirr</div>
                    <div class="text-xs text-gray-600">telebirr</div>
                  </div>
                  <div v-if="selectedPaymentMethod === 'telebirr'" class="absolute -top-1 -right-1 w-5 h-5 bg-green-500 rounded-full flex items-center justify-center">
                    <svg class="w-3 h-3 text-white" fill="currentColor" viewBox="0 0 20 20">
                      <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                    </svg>
                  </div>
                </div>

                <!-- CBE Birr -->
                <div 
                  @click="selectedPaymentMethod = 'cbebirr'"
                  :class="[
                    'relative border-2 rounded-lg p-3 cursor-pointer transition-all hover:scale-105',
                    selectedPaymentMethod === 'cbebirr' 
                      ? 'border-purple-500 bg-purple-50' 
                      : 'border-gray-200 hover:border-gray-300'
                  ]"
                >
                  <div class="text-center">
                    <div class="w-12 h-12 bg-purple-500 rounded-lg mx-auto mb-2 flex items-center justify-center">
                      <span class="text-white font-bold text-sm">CBE</span>
                    </div>
                    <div class="text-xs font-medium text-gray-700">CBE Birr</div>
                    <div class="text-xs text-gray-600">cbebirr</div>
                  </div>
                </div>

                <!-- M-PESA -->
                <div 
                  @click="selectedPaymentMethod = 'mpesa'"
                  :class="[
                    'relative border-2 rounded-lg p-3 cursor-pointer transition-all hover:scale-105',
                    selectedPaymentMethod === 'mpesa' 
                      ? 'border-green-400 bg-green-50' 
                      : 'border-gray-200 hover:border-gray-300'
                  ]"
                >
                  <div class="text-center">
                    <div class="w-12 h-12 bg-green-400 rounded-lg mx-auto mb-2 flex items-center justify-center">
                      <span class="text-white font-bold text-sm">M</span>
                    </div>
                    <div class="text-xs font-medium text-gray-700">M-PESA</div>
                    <div class="text-xs text-gray-600">mpesa</div>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- Order Details (Hidden by default, shown when needed) -->
          <div v-if="showOrderDetails" class="border-t border-gray-200 pt-6 space-y-4">
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Order Number</label>
                <input
                  v-model="formData.order_number"
                  type="text"
                  required
                  class="w-full border border-gray-300 rounded-md px-3 py-2 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                  placeholder="Enter order number"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Amount (ETB)</label>
                <input
                  v-model="formData.amount"
                  type="number"
                  step="0.01"
                  min="0"
                  required
                  class="w-full border border-gray-300 rounded-md px-3 py-2 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                  placeholder="0.00"
                />
              </div>
            </div>
            <div>
              <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
              <input
                v-model="formData.email"
                type="email"
                required
                class="w-full border border-gray-300 rounded-md px-3 py-2 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                placeholder="your@email.com"
              />
            </div>
            <div class="grid grid-cols-2 gap-4">
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Success URL</label>
                <input
                  v-model="formData.success_url"
                  type="url"
                  required
                  class="w-full border border-gray-300 rounded-md px-3 py-2 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                  placeholder="https://example.com/success"
                />
              </div>
              <div>
                <label class="block text-sm font-medium text-gray-700 mb-1">Failure URL</label>
                <input
                  v-model="formData.failure_url"
                  type="url"
                  required
                  class="w-full border border-gray-300 rounded-md px-3 py-2 focus:border-blue-500 focus:ring-1 focus:ring-blue-500"
                  placeholder="https://example.com/failure"
                />
              </div>
            </div>
          </div>

          <!-- Toggle Order Details Button -->
          <div class="text-center">
            <button
              type="button"
              @click="showOrderDetails = !showOrderDetails"
              class="text-blue-600 hover:text-blue-800 text-sm font-medium"
            >
              {{ showOrderDetails ? 'Hide' : 'Show' }} Advanced Options
            </button>
          </div>
        </form>
      </div>

      <!-- Footer Section -->
      <div class="border-t border-gray-200 p-6">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center space-x-2">
            <span class="text-sm text-gray-600">Powered by</span>
            <div class="flex items-center space-x-1">
              <span class="text-blue-600 font-bold text-lg">y</span>
              <span class="text-red-600 font-bold text-lg">agoutpay</span>
            </div>
          </div>
          <div class="w-16 h-8 bg-gray-200 rounded flex items-center justify-center">
            <span class="text-xs text-gray-600 font-semibold">PCI DSS</span>
          </div>
        </div>
        
        <!-- Amount and Pay Button -->
        <div class="flex items-center justify-between">
          <div class="text-2xl font-bold text-gray-900">ETB {{ formData.amount || '0.00' }}</div>
          <button
            @click="handleSubmit"
            :disabled="isLoading || !selectedPaymentMethod"
            class="bg-green-600 text-white font-bold py-3 px-8 rounded-lg hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <span v-if="isLoading" class="inline-flex items-center">
              <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Processing...
            </span>
            <span v-else>Pay With {{ selectedPaymentMethod || 'Wallet' }}</span>
          </button>
        </div>
      </div>
    </div>

    <!-- Payment Form (Hidden) -->
    <form
      v-if="paymentData"
      ref="paymentForm"
      name="paymentForm"
      method="POST"
      enctype="application/x-www-form-urlencoded"
      :action="paymentData.post_url"
      class="hidden"
    >
      <input name="me_id" :value="paymentData.me_id" type="hidden" />
      <input name="merchant_request" :value="paymentData.merchant_request" type="hidden" />
      <input name="hash" :value="paymentData.hash" type="hidden" />
      <input type="submit" name="submit" value="Pay Now" />
    </form>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

interface PaymentFormData {
  order_number: string
  amount: number
  success_url: string
  failure_url: string
  email: string
  mobile_no: string
}

interface PaymentResponse {
  post_url: string
  me_id: string
  merchant_request: string
  hash: string
}

const formData = reactive<PaymentFormData>({
  order_number: 'ORD-' + Date.now(),
  amount: 10.00,
  success_url: 'https://example.com/success',
  failure_url: 'https://example.com/failure',
  email: 'customer@example.com',
  mobile_no: '0911223344'
})

const isLoading = ref(false)
const paymentData = ref<PaymentResponse | null>(null)
const paymentForm = ref<HTMLFormElement>()
const selectedPaymentMethod = ref('telebirr')
const showOrderDetails = ref(false)

const currentTime = computed(() => {
  const now = new Date()
  return now.toLocaleTimeString('en-US', { 
    hour: '2-digit', 
    minute: '2-digit',
    hour12: false 
  })
})

const handleSubmit = async () => {
  try {
    isLoading.value = true
    
    const response = await fetch('http://127.0.0.1:8000/create_payment', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(formData),
    })

    if (!response.ok) {
      throw new Error(`HTTP error! status: ${response.status}`)
    }

    const data: PaymentResponse = await response.json()
    paymentData.value = data
    
    // Auto-submit the payment form
    setTimeout(() => {
      if (paymentForm.value) {
        paymentForm.value.submit()
      }
    }, 100)

  } catch (error) {
    console.error('Payment creation failed:', error)
    alert('Failed to create payment. Please try again.')
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  // Set default values
  if (!formData.order_number) {
    formData.order_number = 'ORD-' + Date.now()
  }
})
</script>

<style scoped>
/* Custom scrollbar for better UX */
::-webkit-scrollbar {
  width: 6px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 3px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}
</style> 