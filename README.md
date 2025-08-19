# Project Name

## Overview
This project integrates a Vue 3 frontend with a Rust backend to process payments via YagoutPay.

## Folder Structure
- `Backend/`: Rust backend using Actix Web.
- `Frontend/`: Vue 3 frontend with TypeScript and Vite.

## Setup Instructions

### Backend
1. Navigate to the backend folder:
   ```bash
   cd Backend
   ```
2. Create a `.env` file with:
   ```env
   MERCHANT_ID=your_merchant_id
   ENCRYPTION_KEY=your_base64_key
   ```
3. Build and run the server:
   ```bash
   cargo run
   ```
The backend will be available at [http://127.0.0.1:8000](http://127.0.0.1:8000).

### Frontend
1. Navigate to the frontend folder:
   ```bash
   cd Frontend
   ```
2. Install dependencies:
   ```bash
   npm install
   ```
3. Run the development server:
   ```bash
   npm run dev
   ```
The frontend will be available at [http://127.0.0.1:5173](http://127.0.0.1:5173).

## Usage
- Open the frontend in the browser.
- Click Checkout Now.
- The frontend sends a request to the backend, which returns encrypted payment data to the frontend and submit for YagoutPay.
