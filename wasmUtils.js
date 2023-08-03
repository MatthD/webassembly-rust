export const appendToBody = (nb) => {
  const text = document.createTextNode(nb);
  document.body.appendChild(text);
};
