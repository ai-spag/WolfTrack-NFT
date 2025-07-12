<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>WolfTrack-NFT Dashboard</title>
    <!-- Tailwind CSS CDN -->
    <script src="https://cdn.tailwindcss.com"></script>
    <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;600;700&display=swap" rel="stylesheet">
    <style>
        body {
            font-family: 'Inter', sans-serif;
            background-color: #0f172a; /* Dark blue-gray background */
            color: #e2e8f0; /* Light gray text */
        }
        /* Custom scrollbar for better aesthetics */
        ::-webkit-scrollbar {
            width: 8px;
        }
        ::-webkit-scrollbar-track {
            background: #1e293b;
            border-radius: 10px;
        }
        ::-webkit-scrollbar-thumb {
            background: #475569;
            border-radius: 10px;
        }
        ::-webkit-scrollbar-thumb:hover {
            background: #64748b;
        }
    </style>
</head>
<body class="min-h-screen flex flex-col items-center p-4 sm:p-6 lg:p-8">

    <!-- Header Section -->
    <header class="w-full max-w-4xl text-center mb-8">
        <h1 class="text-4xl sm:text-5xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-purple-600 mb-2 rounded-lg p-2">
            WolfTrack-NFT Dashboard
        </h1>
        <p class="text-lg sm:text-xl text-gray-300">Your Journey in Agri-Tech, Sports, AI, and Robotics</p>
    </header>

    <!-- Main Content Area -->
    <main class="w-full max-w-4xl bg-gray-800 rounded-xl shadow-lg p-6 sm:p-8 flex flex-col gap-8">

        <!-- Participant Profile Section -->
        <section class="bg-gray-700 p-6 rounded-lg shadow-md">
            <h2 class="text-2xl font-semibold mb-4 text-blue-300">🚀 Participant Profile</h2>
            <div class="flex flex-col sm:flex-row items-center sm:items-start gap-6">
                <img src="https://placehold.co/120x120/334155/e2e8f0?text=Avatar" alt="Participant Avatar" class="w-24 h-24 sm:w-32 sm:h-32 rounded-full border-4 border-purple-500 shadow-lg">
                <div class="text-center sm:text-left">
                    <p class="text-xl font-bold text-white">Participant ID: <span id="participantId">Loading...</span></p>
                    <p class="text-gray-300">Name: <span id="participantName">WolfTrack Alpha</span></p>
                    <p class="text-gray-300">Enrolled Camps: <span id="enrolledCamps">Agri-Tech Summer 2026, AI Robotics Camp</span></p>
                    <p class="text-300">Total Tasks Completed: <span id="tasksCompleted">0</span></p>
                    <p class="text-gray-300">GitHub Contributions: <span id="githubContributions">0</span></p>
                </div>
            </div>
        </section>

        <!-- Soulbound Tokens (SBTs) Section -->
        <section class="bg-gray-700 p-6 rounded-lg shadow-md">
            <h2 class="text-2xl font-semibold mb-4 text-green-300">🔗 Soulbound Tokens (SBTs)</h2>
            <div id="sbtContainer" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                <!-- SBT Cards will be injected here by JS -->
                <div class="bg-gray-800 p-4 rounded-lg shadow-md flex flex-col items-center text-center">
                    <img src="https://placehold.co/100x100/1e293b/e2e8f0?text=SBT+1" alt="SBT Icon" class="w-24 h-24 mb-3 rounded-md">
                    <h3 class="font-bold text-lg text-white mb-1">Certified Drone Operator</h3>
                    <p class="text-sm text-gray-400">Agri-Tech Summer 2026</p>
                    <p class="text-xs text-gray-500 mt-2">Non-transferable credential for drone operation skills.</p>
                </div>
                <div class="bg-gray-800 p-4 rounded-lg shadow-md flex flex-col items-center text-center">
                    <img src="https://placehold.co/100x100/1e293b/e2e8f0?text=SBT+2" alt="SBT Icon" class="w-24 h-24 mb-3 rounded-md">
                    <h3 class="font-bold text-lg text-white mb-1">Robotics Level 1 Achiever</h3>
                    <p class="text-sm text-gray-400">AI Robotics Camp</p>
                    <p class="text-xs text-gray-500 mt-2">Proof of foundational robotics knowledge.</p>
                </div>
            </div>
        </section>

        <!-- Tradable Reward NFTs Section -->
        <section class="bg-gray-700 p-6 rounded-lg shadow-md">
            <h2 class="text-2xl font-semibold mb-4 text-yellow-300">🏆 Tradable Reward NFTs</h2>
            <div id="nftContainer" class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
                <!-- NFT Cards will be injected here by JS -->
                <div class="bg-gray-800 p-4 rounded-lg shadow-md flex flex-col items-center text-center h-full">
                    <img src="https://placehold.co/100x100/1e293b/e2e8f0?text=NFT+1" alt="NFT Image" class="w-24 h-24 mb-3 rounded-md">
                    <h3 class="font-bold text-lg text-white mb-1">Innovation Challenge Winner</h3>
                    <p class="text-sm text-gray-400">Awarded for winning the AI Innovation Challenge.</p>
                    <button class="mt-3 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition duration-200 shadow-md">View on Marketplace</button>
                </div>
                <div class="bg-gray-800 p-4 rounded-lg shadow-md flex flex-col items-center text-center h-full">
                    <img src="https://placehold.co/100x100/1e293b/e2e8f0?text=NFT+2" alt="NFT Image" class="w-24 h-24 mb-3 rounded-md">
                    <h3 class="font-bold text-lg text-white mb-1">Collaboration Master</h3>
                    <p class="text-sm text-gray-400">Recognizing exceptional teamwork in Sports Program.</p>
                    <button class="mt-3 px-4 py-2 bg-purple-600 text-white rounded-md hover:bg-purple-700 transition duration-200 shadow-md">View on Marketplace</button>
                </div>
            </div>
        </section>

        <!-- Interactive Actions Section -->
        <section class="bg-gray-700 p-6 rounded-lg shadow-md">
            <h2 class="text-2xl font-semibold mb-4 text-red-300">⚡ Actions</h2>
            <div class="flex flex-col sm:flex-row gap-4">
                <button id="scanQrBtn" class="flex-1 px-6 py-3 bg-blue-500 text-white font-semibold rounded-lg hover:bg-blue-600 transition duration-200 shadow-lg flex items-center justify-center">
                    <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path></svg>
                    Scan Task QR Code
                </button>
                <button id="checkGithubBtn" class="flex-1 px-6 py-3 bg-indigo-500 text-white font-semibold rounded-lg hover:bg-indigo-600 transition duration-200 shadow-lg flex items-center justify-center">
                    <svg class="w-6 h-6 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"></path></svg>
                    Check GitHub Submissions
                </button>
            </div>
            <div id="messageBox" class="mt-6 p-4 bg-gray-900 text-gray-200 rounded-md hidden">
                <!-- Messages will appear here -->
            </div>
        </section>

        <!-- About WolfTrack-NFT Section -->
        <section class="bg-gray-700 p-6 rounded-lg shadow-md">
            <h2 class="text-2xl font-semibold mb-4 text-purple-300">ℹ️ About WolfTrack-NFT</h2>
            <div class="prose prose-invert max-w-none text-gray-300">
                <p><strong>WolfTrack-NFT</strong> is an innovative, youth-focused Web3 platform designed to foster engagement and skill development in critical future-focused domains: Agri-Tech, Sports, Artificial Intelligence, and Robotics. Leveraging the high performance and low transaction costs of the **Solana blockchain**, with core logic built in **Rust smart contracts**, WolfTrack-NFT creates a dynamic and rewarding ecosystem for participants in specialized summer camps and year-round programs through the strategic use of Non-Fungible Tokens (NFTs). This system is also specifically designed to **teach on-chain and crypto literacy for students**, providing a hands-on experience with blockchain technology.</p>
                <p>&nbsp;</p> <!-- Added line spacing here -->
                <p>The platform utilizes a dual-token system (non-transferable **SBTs** for achievements and tradable **NFTs** for rewards), features gamified task tracking via QR codes and GitHub submissions, incorporates dynamic NFT visuals that evolve with participant progress, and integrates real-world mentorship and access rewards.</p>
            </div>
        </section>

    </main>

    <script>
        // --- Conceptual JavaScript for Frontend Interactions ---

        // Function to display messages in the message box
        function showMessage(message, type = 'info') {
            const messageBox = document.getElementById('messageBox');
            messageBox.textContent = message;
            messageBox.className = 'mt-6 p-4 rounded-md'; // Reset classes
            if (type === 'info') {
                messageBox.classList.add('bg-blue-900', 'text-blue-100');
            } else if (type === 'success') {
                messageBox.classList.add('bg-green-900', 'text-green-100');
            } else if (type === 'error') {
                messageBox.classList.add('bg-red-900', 'text-red-100');
            }
            messageBox.classList.remove('hidden');
            setTimeout(() => {
                messageBox.classList.add('hidden');
            }, 5000); // Hide after 5 seconds
        }

        // Simulate fetching participant data
        function fetchParticipantData() {
            // In a real app, this would fetch from Solana or a backend API
            document.getElementById('participantId').textContent = 'GH7d8f...jK9L0'; // Example Solana Pubkey
            document.getElementById('tasksCompleted').textContent = '15';
            document.getElementById('githubContributions').textContent = '7';
            // Populate SBTs and NFTs dynamically if needed
        }

        // Event Listeners for buttons
        document.getElementById('scanQrBtn').addEventListener('click', () => {
            showMessage('Simulating QR Code scan... (This would open a camera/input)', 'info');
            // In a real app, this would trigger a camera API or prompt for QR data
            // Then, it would send data to the backend service (Java) for validation
            // and subsequent Solana transaction.
            setTimeout(() => {
                showMessage('Task "Agri-Tech Field Prep" completed and recorded!', 'success');
                // Update UI based on new data
                document.getElementById('tasksCompleted').textContent = parseInt(document.getElementById('tasksCompleted').textContent) + 1;
            }, 2000);
        });

        document.getElementById('checkGithubBtn').addEventListener('click', () => {
            showMessage('Checking recent GitHub submissions... (Connecting to backend)', 'info');
            // In a real app, this would call the Java backend service's checkGitHubSubmission method
            // The backend would then interact with GitHub API and potentially Solana.
            setTimeout(() => {
                const isNewContribution = Math.random() > 0.5; // Simulate success/failure
                if (isNewContribution) {
                    showMessage('New GitHub contribution detected! SBT "AI Model Contributor" minted!', 'success');
                    document.getElementById('githubContributions').textContent = parseInt(document.getElementById('githubContributions').textContent) + 1;
                    // Add new SBT card dynamically
                    const sbtContainer = document.getElementById('sbtContainer');
                    const newSbtCard = `
                        <div class="bg-gray-800 p-4 rounded-lg shadow-md flex flex-col items-center text-center">
                            <img src="https://placehold.co/100x100/1e293b/e2e8f0?text=New+SBT" alt="New SBT Icon" class="w-24 h-24 mb-3 rounded-md">
                            <h3 class="font-bold text-lg text-white mb-1">AI Model Contributor</h3>
                            <p class="text-sm text-gray-400">Recognizing your code contributions to AI projects.</p>
                            <p class="text-xs text-gray-500 mt-2">Minted: ${new Date().toLocaleDateString()}</p>
                        </div>
                    `;
                    sbtContainer.insertAdjacentHTML('beforeend', newSbtCard);

                } else {
                    showMessage('No new eligible GitHub submissions found at this time.', 'info');
                }
            }, 3000);
        });

        // Initial data load on page load
        document.addEventListener('DOMContentLoaded', fetchParticipantData);
    </script>
</body>
</html>
