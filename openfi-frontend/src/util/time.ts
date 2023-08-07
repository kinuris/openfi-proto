const secondsPerDay = 24 * 60 * 60;
const secondsPerHour = 60 * 60;
const secondsPerMinute = 60;

export function secondsToFormattedTime(seconds: number): string {
    let totalSeconds = seconds;

    const days = Math.floor(totalSeconds / secondsPerDay);
    totalSeconds -= days * secondsPerDay;

    const hours = Math.floor(totalSeconds / secondsPerHour);
    totalSeconds -= hours * secondsPerHour;

    const minutes = Math.floor(totalSeconds / secondsPerMinute);
    totalSeconds -= minutes * secondsPerMinute;

    return `${days.toString().padStart(2, "0")}D : ${hours.toString().padStart(2, "0")}H : ${minutes.toString().padStart(2, "0")}M : ${totalSeconds.toString().padStart(2, "0")}S`;
}

export function secondsToTime(seconds: number) {
    let totalSeconds = seconds;



    const days = Math.floor(totalSeconds / secondsPerDay);
    totalSeconds -= days * secondsPerDay;

    const hours = Math.floor(totalSeconds / secondsPerHour);
    totalSeconds -= hours * secondsPerHour;

    const minutes = Math.floor(totalSeconds / secondsPerMinute);
    totalSeconds -= minutes * secondsPerMinute;

    return {
        days,
        hours,
        minutes,
        seconds: totalSeconds
    }
}

export function DHMToSeconds(d: number, h: number, m: number) {
    return (d * secondsPerDay) + (h * secondsPerHour) + (m * secondsPerMinute)
}