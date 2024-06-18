import { StatusType } from "@/utilities";
import { computed } from "vue";

export default function(getStatus) {
  console.log(getStatus, "getStatus");
  const statusText = computed({
    get() {
      switch (getStatus) {
      case StatusType.Approving:
        return "Approve Transaction";
      case StatusType.Sending:
        return "Sending Transaction...";
      case StatusType.Applying:
        return "Applying the chosen effect...";
      case StatusType.DeployingToIPFS:
        return "Uploading the result to IPFS...";
      case StatusType.Minting:
        return "NFT Minting...";
      case StatusType.Minted:
        return "NFT successfully Minted!";
      default:
        return "";
      }
    },
  });
  
  return {
    statusText: statusText.value,
    StatusType,
  };
}
