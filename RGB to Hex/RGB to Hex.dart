String rgbToHex(String rgbStr) {
  // remove "rgb(" and ")" then split into numbers
  final values = rgbStr
      .replaceAll(RegExp(r'rgb|\(|\)|\s'), '')
      .split(',')
      .map(int.parse)
      .toList();

  final r = values[0];
  final g = values[1];
  final b = values[2];

  // format each value as 2-digit hex
  return '#${r.toRadixString(16).padLeft(2, '0')}' +
         '${g.toRadixString(16).padLeft(2, '0')}' +
         '${b.toRadixString(16).padLeft(2, '0')}';
}



