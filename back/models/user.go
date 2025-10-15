package models

import (
	"time"

	"gorm.io/gorm"
)

type Role string

const (
	RoleEmployee Role = "employee"
	RoleManager  Role = "manager"
	RoleAdmin    Role = "admin"
)

type User struct {
	ID           uint           `json:"id" gorm:"primaryKey"`
	Email        string         `json:"email" gorm:"uniqueIndex;size:255;not null"`
	PasswordHash string         `json:"-" gorm:"size:255;not null"`
	FirstName    string         `json:"first_name" gorm:"size:100;not null"`
	LastName     string         `json:"last_name" gorm:"size:100;not null"`
	PhoneNumber  string         `json:"phone_number" gorm:"size:20"`
	Role         Role           `json:"role" gorm:"type:varchar(20);default:'employee';not null"`
	CreatedByID  *uint          `json:"created_by_id" gorm:"index"`
	CreatedBy    *User          `json:"created_by,omitempty" gorm:"foreignKey:CreatedByID"`
	TeamID       *uint          `json:"team_id" gorm:"index"`
	Team         *Team          `json:"team,omitempty" gorm:"foreignKey:TeamID"`
	Teams        []Team         `json:"teams,omitempty" gorm:"many2many:manager_teams;"`
	CreatedAt    time.Time      `json:"created_at"`
	UpdatedAt    time.Time      `json:"updated_at"`
	DeletedAt    gorm.DeletedAt `json:"-" gorm:"index"`
}

func (User) TableName() string {
	return "users"
}
