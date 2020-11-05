import { promisified } from "tauri/api/tauri";

const getEntries = (path: String) => {
  return promisified({
    cmd: "getEntries",
    path: path,
  })
    .then((response: any) => {
      return response;
    })
    .catch((error: any) => {
      throw error;
    });
};

export default getEntries;
