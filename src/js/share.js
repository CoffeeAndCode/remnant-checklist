import { shareApp } from "./stats.js";

export function canShare() {
  return Boolean(navigator.share);
}

export function share(title, text, url) {
  const data = {
    text,
    title,
    url,
  };

  if (canShare()) {
    navigator.share(data).then(shareApp, console.error);
    return true;
  } else {
    return false;
  }
}
