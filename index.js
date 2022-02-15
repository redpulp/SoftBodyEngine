const getElementById = (elements, id) => {
  elements.find((el) => el.id === id || getElementById(el.children, id));
};
