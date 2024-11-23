

export function convertDateToMMDDYY(dateString: string | null) {
  if (dateString) {
    const [year, month, day] = dateString.split('-');
    return `${month}${day}${year}`;
  }
  return null
}
