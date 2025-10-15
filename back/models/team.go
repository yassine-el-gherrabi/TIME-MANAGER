package models

import (
	"time"

	"gorm.io/gorm"
)

type Team struct {
	ID          uint           `json:"id" gorm:"primaryKey"`
	Name        string         `json:"name" gorm:"size:100;not null"`
	Description string         `json:"description" gorm:"size:500"`
	CreatedByID uint           `json:"created_by_id" gorm:"not null;index"`
	CreatedBy   *User          `json:"created_by,omitempty" gorm:"foreignKey:CreatedByID"`
	Employees   []User         `json:"employees,omitempty" gorm:"foreignKey:TeamID"`
	Managers    []User         `json:"managers,omitempty" gorm:"many2many:manager_teams;"`
	CreatedAt   time.Time      `json:"created_at"`
	UpdatedAt   time.Time      `json:"updated_at"`
	DeletedAt   gorm.DeletedAt `json:"-" gorm:"index"`
}

func (Team) TableName() string {
	return "teams"
}
