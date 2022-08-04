-- SELECT MAX(datediff(`return_date`, `rental_date`)) AS `max`
-- FROM `rental`;
SELECT MAX(datediff(return_date, rental_date))
FROM rental;