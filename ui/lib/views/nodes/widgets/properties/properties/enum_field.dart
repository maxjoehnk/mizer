import 'package:flutter/material.dart';
import 'package:mizer/widgets/controls/select.dart';

import 'field.dart';

class EnumField extends StatelessWidget {
  final String label;
  final List<SelectOption<int>> items;
  final int? initialValue;
  final Function(int) onUpdate;

  EnumField({ required this.label, required this.items, this.initialValue, required this.onUpdate });

  @override
  Widget build(BuildContext context) {
    return Field(
      label: label,
      child: MizerSelect<int>(
        value: this.initialValue,
        options: this.items,
        onChanged: (value) => onUpdate(value),
      ),
    );
  }
}

class EnumItem<T> {
  final String label;
  final T value;

  EnumItem({ required this.label, required this.value });
}
