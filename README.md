# Overview

A high-performance Rust backend serving two AI-powered endpoints:

- Poem Generation (/generate-poem)

- Text Paraphrasing (/paraphrase)

Built with Actix-web and integrated with DeepInfra's LLM API.


## ✨ Features
- Blazing Fast ⚡ - Rust-powered performance

- Simple API - Consistent request/response format

- Secure - CORS configured for production

- Scalable - Ready for deployment

## 🛠️ Tech Stack
 - Rust (Actix-web framework)

- DeepInfra (Llama-2-70b-chat model)

- Serde (JSON serialization)

- Reqwest (HTTP client)

## 🔧 Installation

### Clone repository
```bash
    git clone https://github.com/henok-projects/Paraphrasing-backend.git
```
### Navigate to backend
```bash
  cd Paraphrasing-backend
```
### Install Rust dependencies
```bash 
    cargo build
``` 
### Set environment variables
Create a .env file in the backend root with:
```bash
    OPENAI_API_KEY=your_deepinfra_api_key_here
```

### Build and run
 - cargo run --release

### Access the Application
- Frontend will run on: http://localhost:3000
- Backend API will run on: http://localhost:8080

## 🌐 API Endpoints
#### Endpoints with Request and Response format
- /generate-poem	POST	{ prompt: string }	{ content: string }
- /paraphrase	POST	{ prompt: string }	{ content: string }

## 🚀 Deployment
# Production build
cargo build --release
