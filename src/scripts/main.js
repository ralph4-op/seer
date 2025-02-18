//create an instance of orbitdb that will take in users signatures and ask them for a username.
// Function to flag posts with restricted content
function filterRestrictedContent(post) {
    const restrictedKeywords = ['gore', 'illegal-content'];
    if (restrictedKeywords.some(keyword => post.content.includes(keyword))) {
      return { ...post, status: 'flagged' };
    }
    return post;
  }
  
  // Example: Adding a post with automated filtering
  let newPost = {
    id: 'unique-id',
    user: 'user-address',
    content: 'This is illegal-content!',
    status: 'pending',
    createdAt: Date.now(),
  };
  newPost = filterRestrictedContent(newPost);
  await postsDb.put(newPost);
  console.log(`Post status: ${newPost.status}`);
  


export async function addUserProfile(signature, username) {
    if (!db) {
        throw new Error('OrbitDB not initialized');
    }

    const existingUser = await db.get(username);
    if (existingUser) {
        throw new Error('Username already taken');
    }

    await db.put(username, { signature, username });
    console.log('User profile added:', username);
}

export async function getUserProfile(username) {
    if (!db) {
        throw new Error('OrbitDB not initialized');
    }

    return await db.get(username);
}

// Initialize OrbitDB when the app loads
document.addEventListener('DOMContentLoaded', async () => {
    await initOrbitDB();
    console.log('OrbitDB is ready');
});