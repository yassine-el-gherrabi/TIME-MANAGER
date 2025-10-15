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

// l'admin créé un manager et peut egalement créer un employé lui seul peut creer et c'est lui qui affilie un employé à une team (il a le crud complet)
// le manager est juste affilié à une team et peut etre affilié à plusieurs teams (juste la vue sur sa ou ses teams qui lui sont affiliées pas de delete create update)
// l'employé est affilié à une team et ne peut pas etre affilié à plusieurs teams
// donc un admin creer des teams à lui et des employés et des managers
// un manager est affilié à une team et peut voir les employés de sa team
// un employé est affilié à une team et peut voir les autres employés de sa team
// un employé ne peut pas voir les managers ni les admins
// un manager ne peut pas voir les admins
// un admin peut tout voir
//en gros un manager et un emplyé est forcement affilié à un admin
//un employé est forcement affilié à un manager
//un admin peut créer des managers et des employés
