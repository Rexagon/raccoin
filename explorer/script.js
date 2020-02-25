const API_URL = 'http://127.0.0.1:11000';

const getBlocks = () => fetch(`${API_URL}/blocks`).then(data => data.json());
const createBlock = data => fetch(`${API_URL}/blocks`, {
  method: 'POST',
  body: JSON.stringify(data),
  headers: {
    'Content-Type': 'application/json'
  }
}).then(data => data.json());

const createBlockElement = (text) => {
  let block = document.createElement('li');
  let pre = document.createElement('pre');
  block.appendChild(pre);

  pre.innerText = text;

  return block;
};

const updateBlockList = () => getBlocks().then(blocks => {
  const blocksListElement = document.getElementById('blocks-list');
  blocksListElement.textContent = '';

  blocks.map(block => ({
      ...block,
      timestamp: new Date(block.timestamp)
    }))
    .sort((a, b) => b.timestamp.getTime() - a.timestamp.getTime())
    .forEach(block => blocksListElement.appendChild(createBlockElement(JSON.stringify(block, undefined, 4))));
});

document.addEventListener('DOMContentLoaded', async () => {
  await updateBlockList();

  const blockForm = document.getElementById('block-form');
  blockForm.addEventListener('submit', e => {
    e.preventDefault();

    const formData = new FormData(blockForm);
    const data = formData.get("data");

    if (data.length === 0) {
      return;
    }

    blockForm.reset();
    createBlock(data);
  });
});
