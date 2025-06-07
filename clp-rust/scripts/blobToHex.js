const toHexString = (byteArray) => {
  return Array.from(byteArray, function (byte) {
    return ('0' + (byte & 0xff).toString(16)).slice(-2);
  })
    .join('')
    .toUpperCase();
};

function toVec(byteArray) {
  let content = Array.from(byteArray, function (byte) {
    return (byte & 0xff).toString(10);
  })
    .join(';')
    .toUpperCase();
  return 'vec{' + content + '}';
}

// 获取命令行参数
const args = process.argv.slice(2);

// 打印参数
// console.log(toHexString(Buffer.from(args[2].substring(2, 97).split('\\').join(''), 'hex')));
console.log(toVec(Buffer.from(args[2].substring(2, 97).split('\\').join(''), 'hex')));
