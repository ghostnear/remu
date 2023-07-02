pub fn clamp<TData>(input: TData, min: TData, max: TData) -> TData
where
	TData: PartialOrd<TData>
{
	if input < min
	{
		return min;
	}

	if input > max
	{
		return max;
	}

	return input;
}
