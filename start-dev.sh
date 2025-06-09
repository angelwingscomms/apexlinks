#!/bin/bash

# ApexLinks Development Startup Script

echo "🌙 Starting ApexLinks Development Environment..."

# Check if Shuttle CLI is installed
if ! command -v shuttle &> /dev/null; then
    echo "⚠️  Shuttle CLI is not installed. Please install it first:"
    echo "   cargo install cargo-shuttle"
    exit 1
fi

# Check if Qdrant is running
echo "📊 Checking Qdrant connection..."
if ! curl -s http://localhost:6333/readyz > /dev/null; then
    echo "⚠️  Qdrant is not running. Please start Qdrant first:"
    echo "   docker run -p 6333:6333 qdrant/qdrant"
    exit 1
fi

echo "✅ Qdrant is running"

# Start backend in background
echo "🦀 Starting Rust backend..."
cd 144
shuttle run &
BACKEND_PID=$!
cd ..

# Wait a moment for backend to start
sleep 3

# Start frontend
echo "⚡ Starting SvelteKit frontend..."
cd apexlinks
npm run dev &
FRONTEND_PID=$!
cd ..

echo "🚀 ApexLinks is starting up!"
echo "📱 Frontend: http://localhost:5173"
echo "🔧 Backend: http://localhost:8000"
echo ""
echo "Press Ctrl+C to stop all services"

# Wait for user to stop
trap "echo '🛑 Stopping services...'; kill $BACKEND_PID $FRONTEND_PID; exit" INT
wait 