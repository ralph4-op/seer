//create an instance of orbitdb that will take in users signatures and ask them for a username.
import { createHelia, libp2pDefaults } from 'helia'
import { createOrbitDB, Identities, useIdentityProvider } from '@orbitdb/core'
import * as OrbitDBIdentityProviderEthereum from '@orbitdb/identity-provider-ethereum'
import { createOrbitDB, useAccessController } from '@orbitdb/core'
import { Libp2pOptions } from './config/libp2p.js'
import { createLibp2p } from 'libp2p'
import { createHelia } from 'helia'
import { LevelBlockstore } from 'blockstore-level'

const libp2pOptions = libp2pDefaults()
const ipfs = await createHelia({ libp2p: libp2pOptions })

useIdentityProvider(OrbitDBIdentityProviderEthereum)
const provider = OrbitDBIdentityProviderEthereum({ wallet })

const identities = await Identities({ ipfs })
const identity = await identities.createIdentity({ id: 'userA', provider })

await createOrbitDB({ ipfs, identities, identity })

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
  