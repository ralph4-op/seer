import { createHelia, libp2pDefaults } from 'helia'
import { createOrbitDB, Identities, useIdentityProvider } from '@orbitdb/core'
import * as OrbitDBIdentityProviderEthereum from '@orbitdb/identity-provider-ethereum'
import { createOrbitDB, useAccessController } from '@orbitdb/core'
import { Libp2pOptions } from './config/libp2p.js'
import { createLibp2p } from 'libp2p'
import { createHelia } from 'helia'
import { LevelBlockstore } from 'blockstore-level'

export async function initOrbitDB() {
    const blockstore = new LevelBlockstore('./ipfs');
    const ipfs = await createHelia({ blockstore });

    useIdentityProvider(OrbitDBIdentityProviderEthereum);
    const identities = await Identities({ ipfs });
    const identity = await identities.createIdentity({ provider: OrbitDBIdentityProviderEthereum });

    orbitdb = await createOrbitDB({ ipfs, identities, identity });
    db = await orbitdb.keyvalue('user-profiles');
    console.log('OrbitDB initialized:', db.address.toString());
}

const libp2pOptions = libp2pDefaults()
const ipfs = await createHelia({ libp2p: libp2pOptions })

useIdentityProvider(OrbitDBIdentityProviderEthereum)
const provider = OrbitDBIdentityProviderEthereum({ wallet })

const identities = await Identities({ ipfs })
const identity = await identities.createIdentity({ id: 'userA', provider })

await createOrbitDB({ ipfs, identities, identity })

//create the right access controls the database should require the admins signature especially for validating
// the status of the posts, profiles and comments.