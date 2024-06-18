import axios from "axios";

const api  = axios.create({
  baseURL: process.env.VUE_APP_API_URL,
  timeout: 60000,
});

export default api;

export async function getFromBackIPFS (uriIPFS) {
  let result = null;

  try {
    result = await api.get(`/ipfs/cat?ipfs_addr=${uriIPFS}`);
    console.log(result, "RESULT");
  } catch(err) {
    console.log(err, "error uploadtoIPFS");
  }

  return result ? result.data : null;
}

export async function getAsBlobFromBackIPFS (uriIPFS) {
  let result = null;

  try {
    result = await api.get(`/ipfs/cat?ipfs_addr=${uriIPFS}`, {responseType: "blob"});
    console.log(result, "RESULT");
  } catch(err) {
    console.log(err, "error uploadtoIPFS");
  }

  return result ? result.data : null;
}


export async function uploadtoIPFS (data, isJSON) {
  let result = null;
  const formData = new FormData();
  let file = new Blob([JSON.stringify(data)], {type: "application/json"});

  if (!isJSON) {
    const fetchUrl = await fetch(data);
    file = await fetchUrl.blob();
  }

  formData.append("payload", file);

  try {
    result = await api.post("/ipfs/upload",  formData);
    console.log(result, "RESULT");
  } catch(err) {
    console.log(err, "error uploadtoIPFS");
  }

  return result ? `https://ipfs.io/${result.data.replace(":/", "")}` : null;
}


export async function applyNFTsEffect (effectData) {
  let result = null;

  try {
    result = await api.post("/effects/applyEffect", effectData, {responseType: "blob"});
  } catch(err) {
    console.log(err, "error applyNFTsEffect");
  }
  
  let cid = null;
  // for immidiate display of applied effect from IPFS
  let bundleImageTempURL = null;

  console.log(result.headers.contenturl, "result.headers.contenturl APPLY NFT EFFECT");
  if (typeof result.headers.contenturl === "string") {
    cid = `https://ipfs.io/${result.headers.contenturl.replace(":/", "")}`;
    bundleImageTempURL = URL.createObjectURL(result.data);
  } else {
    throw new Error();
  }
  console.log(cid, "CID APPLY NFT EFFECT");

  return cid ? { cid, hashBlob: bundleImageTempURL } : null;
}