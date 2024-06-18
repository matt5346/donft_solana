import { CID_RE } from "@/utilities";
import { uploadtoIPFS, getFromBackIPFS, getAsBlobFromBackIPFS } from "@/api";
import { NFTStorage } from "nft.storage/dist/bundle.esm.min.js";
const API_KEY = process.env.VUE_APP_NFT_STORAGE_API_KEY;

import { getParsedNftAccountsByOwner } from "@nfteyez/sol-rayz";

const client = new NFTStorage({
  token: API_KEY,
});

export async function loadAllNFTs(solanaInstance, walletInstance, commit) {
  try {
    console.log(solanaInstance, walletInstance, "loadAllNFTs");
    const provider = walletInstance;
    let ownerToken = provider.publicKey.toString();
    let nfts = await getParsedNftAccountsByOwner({
      publicAddress: ownerToken,
      connection: solanaInstance,
      serialization: true,
    });
    commit("SET_NFTS_LOADED", true);
    console.log("nfts", nfts);
    return nfts;
  } catch (error) {
    console.log(error);
  }
}

// todo: add custom mintNFT + upload by BACK
// metaplex library can not read uri as JSON.stringified
async function pushObjectToIpfs(ipfsInstance, object) {
  let cid = null;
  try {
    let file = new Blob([JSON.stringify(object)], {type: "application/json"});
    cid = await client.storeBlob(file);
  } catch(err) {
    console.log(err, "err pushObjectToIpfs");
  }
  return cid;
}

export async function deployNFTtoIPFS(ipfsInstance, meta, isImageDeployed) {
  let imageCID = meta.image;
  console.log(isImageDeployed, "is RANDOM");

  if (!isImageDeployed) {
    const cid = await uploadtoIPFS(meta.image, false);
    imageCID = cid;
  }

  // let meta = JSON.parse(JSON.stringify(oldMeta));
  // meta.animation_url = `ipfs://${imageCID}`;
  let uriJSON = {
    ...meta,
    image: imageCID,
    properties: {
      ...meta.properties,
      files: [
        {
          uri: imageCID,
          type: "image/*"
        }
      ]
    }
  };
  // current type https://ipfs.io/ipfs/{cid}/file
  const metaDataCID = await pushObjectToIpfs(ipfsInstance, uriJSON);
  console.log(metaDataCID, "metaDataCID");
  return `https://ipfs.io/ipfs/${metaDataCID}`;
}

export async function getImageForTokenByURI(ipfsInstance, imageAddress, getIPFSurl) {
  let image;
  console.log(imageAddress, "getImageForTokenByURI");
  if (imageAddress) {
    if (imageAddress.startsWith("ipfs") || imageAddress.startsWith("https://ipfs"))  {
      let cid = CID_RE.exec(imageAddress)?.[0];

      let localImageURL = await getDataFromIPFS(ipfsInstance, cid, getIPFSurl);

      image = localImageURL;
    } else {
      image = imageAddress;
    }
  }
  return image;
}

async function getDataFromIPFS(ipfsInstance, cid, getIPFSurl) {
  let tokenData = null;
  try {
    if (cid === "" || cid === null || cid === undefined) {
      return;
    }
    tokenData = await getFromBackIPFS(cid);

    if (getIPFSurl) {
      return tokenData;
    }
    
    if (tokenData.image.startsWith("ipfs") || tokenData.image.startsWith("https://ipfs") || tokenData.image.endsWith("link/"))  {
      let cid = CID_RE.exec(tokenData.image)?.[0];
      let data = {
        ...tokenData,
        image: null,
      };

      let blobImage = null;

      // nft.storage return images with /file in the end
      // server on apply effect return image without /file
      // possibly will be fixed later
      if (tokenData.image.endsWith("/file")) {
        blobImage = await getAsBlobFromBackIPFS(`${cid}/file`);
      } else {
        blobImage = await getAsBlobFromBackIPFS(cid);
      }

      data.image = URL.createObjectURL(blobImage);

      return data;
    } else {
      return tokenData;
    }
  } catch (e) {
    console.log(e, `getImageFromIpfs ERROR ${cid}`);
  }
}

// async function getImageFromIpfs(ipfsInstance, cid) {
//   let blob = null;
//   try {
//     blob = await loadFileFromIPFS(ipfsInstance, cid, 6000);
//   } catch (e) {
//     console.log(e, `getImageFromIpfs ERROR ${cid}`);
//   }
//   return blob ? URL.createObjectURL(blob) : null;
// }

// async function loadFileFromIPFS(ipfs, cid, timeout) {
//   if (cid === "" || cid === null || cid === undefined) {
//     return;
//   }
//   let content = [];
//   for await (const buff of ipfs.get(cid, {timeout})) {
//     if (buff) {
//       content.push(buff);
//     }
//   }
//   let archivedBlob = new Blob(content, {type: "application/x-tar"});
//   let archiveArrayBuffer = await archivedBlob.arrayBuffer();
//   let archive = (await untar(archiveArrayBuffer))?.[0];

//   return archive.blob;
// }